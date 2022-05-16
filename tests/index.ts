import { expect } from "chai";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { sleep } from "./utils";
import { Context } from "./ctx";
import {
  registerStudent,
  initialize,
  setGrade,
  endTask,
  expelStudent,
} from "./api";

chai.use(chaiAsPromised);

const ctx = new Context();

before(async () => {
  await ctx.setup();
});

describe("instructions", () => {
  it("initialize", async () => {
    await initialize(ctx);

    const dao = await ctx.program.account.dao.fetch(ctx.dao);
    expect(dao.bump).to.gt(200);
    expect(dao.mntrMint).to.eql(ctx.mntrMint);
    expect(dao.authority).to.eql(ctx.daoAuthority.publicKey);
  });

  it("registerStudent", async () => {
    const mentors = [ctx.mentor1.publicKey, ctx.mentor2.publicKey];
    const totalTasks = 2;
    const taskDuration = 10;

    await expect(
      registerStudent(ctx, ctx.student1, mentors, 0, taskDuration)
    ).to.be.rejectedWith("Zero");

    await registerStudent(ctx, ctx.student1, mentors, totalTasks, taskDuration);

    const student = await ctx.program.account.student.fetch(
      await ctx.student(ctx.student1.publicKey)
    );
    expect(student.bump).to.gt(200);
    expect(student.totalTasks).to.eql(totalTasks);
    expect(student.taskDuration).to.eql(taskDuration);
    expect(student.currentTaskStartTs).to.gt(new Date().getSeconds() - 5);
    expect(
      // @ts-ignore
      student.currentGrades.map((g: any) => {
        g.maxGrade = g.maxGrade.toNumber();
        return g;
      })
    ).to.eql(
      mentors.map((mentor) => {
        return { mentor, setGrade: null, maxGrade: 0 };
      })
    );
  });

  it("setGrade", async () => {
    await expect(
      setGrade(ctx, ctx.mentor1, ctx.student1.publicKey, 6)
    ).to.be.rejectedWith("NotEnoughPower");

    await expect(
      setGrade(ctx, ctx.mentor3, ctx.student1.publicKey, 5)
    ).to.be.rejectedWith("NotMentorOfStudent");

    await setGrade(ctx, ctx.mentor1, ctx.student1.publicKey, 0);
    await setGrade(ctx, ctx.mentor1, ctx.student1.publicKey, 4);

    const student = await ctx.program.account.student.fetch(
      await ctx.student(ctx.student1.publicKey)
    );
    expect(
      // @ts-ignore
      student.currentGrades.map(({ mentor, setGrade, maxGrade }) => {
        return {
          mentor,
          setGrade: setGrade ? setGrade.toNumber() : setGrade,
          maxGrade: maxGrade.toNumber(),
        };
      })
    ).to.eql([
      {
        mentor: ctx.mentor1.publicKey,
        setGrade: 4,
        maxGrade: 5,
      },
      {
        mentor: ctx.mentor2.publicKey,
        setGrade: null,
        maxGrade: 0,
      },
    ]);
  });

  it("endTask", async () => {
    await expect(endTask(ctx, ctx.student1.publicKey)).to.be.rejectedWith(
      "TaskTimelock"
    );

    await sleep(10000);

    await expect(endTask(ctx, ctx.student1.publicKey)).to.be.rejectedWith(
      "NotAllMentorsHaveVoted"
    );

    await setGrade(ctx, ctx.mentor2, ctx.student1.publicKey, 8);

    await endTask(ctx, ctx.student1.publicKey);
  });

  it("expelStudent", async () => {
    await expect(
      expelStudent(ctx, ctx.mentor3, ctx.student1.publicKey)
    ).to.be.rejectedWith("NotMentorOfStudent");

    await expelStudent(ctx, ctx.mentor1, ctx.student1.publicKey);
  });
});
