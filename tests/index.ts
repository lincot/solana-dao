import { expect } from "chai";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { sleep } from "./utils";
import { Context } from "./ctx";
import {
  registerStudent,
  initializeAcademy,
  setGrade,
  endTask,
  expelStudent,
} from "./academyAPI";
import { employ, initializeCompany } from "./companyAPI";

chai.use(chaiAsPromised);

const ctx = new Context();

before(async () => {
  await ctx.setup();
});

describe("company", () => {
  it("initialize", async () => {
    await initializeCompany(ctx);

    const company = await ctx.companyProgram.account.company.fetch(ctx.company);
    expect(company.bump).to.gt(200);
    expect(company.authority).to.eql(ctx.companyAuthority.publicKey);
  });

  it("employ", async () => {
    const salary = 10000;

    await employ(ctx, ctx.mentor3, salary);

    const employee = await ctx.companyProgram.account.employee.fetch(
      await ctx.employee(ctx.mentor3.publicKey)
    );
    expect(employee.bump).to.gt(200);
    expect(employee.salary.toNumber()).to.eql(salary);
  });
});

describe("academy", () => {
  it("initialize", async () => {
    await initializeAcademy(ctx);

    const academy = await ctx.academyProgram.account.academy.fetch(ctx.academy);
    expect(academy.bump).to.gt(200);
    expect(academy.mntrMint).to.eql(ctx.mntrMint);
    expect(academy.authority).to.eql(ctx.companyAuthority.publicKey);
  });

  it("registerStudent", async () => {
    const mentors = [ctx.mentor1.publicKey, ctx.mentor2.publicKey];
    const totalTasks = 2;
    const taskDuration = 10;

    await expect(
      registerStudent(ctx, ctx.student1, mentors, 0, taskDuration)
    ).to.be.rejectedWith("Zero");

    await registerStudent(ctx, ctx.student1, mentors, totalTasks, taskDuration);

    const student = await ctx.academyProgram.account.student.fetch(
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
    ).to.be.rejectedWith("NotEnoughMNTR");

    await expect(
      setGrade(ctx, ctx.mentor3, ctx.student1.publicKey, 5)
    ).to.be.rejectedWith("NotMentorOfStudent");

    await setGrade(ctx, ctx.mentor1, ctx.student1.publicKey, 0);
    await setGrade(ctx, ctx.mentor1, ctx.student1.publicKey, 4);

    const student = await ctx.academyProgram.account.student.fetch(
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
    await expect(endTask(ctx, ctx.student1)).to.be.rejectedWith("TaskTimelock");

    await sleep(10000);

    await expect(endTask(ctx, ctx.student1)).to.be.rejectedWith(
      "NotAllMentorsHaveVoted"
    );

    await setGrade(ctx, ctx.mentor2, ctx.student1.publicKey, 8);

    await endTask(ctx, ctx.student1);
  });

  it("expelStudent", async () => {
    await expect(
      expelStudent(ctx, ctx.mentor3, ctx.student1.publicKey)
    ).to.be.rejectedWith("NotMentorOfStudent");

    await expelStudent(ctx, ctx.mentor1, ctx.student1.publicKey);
  });
});
