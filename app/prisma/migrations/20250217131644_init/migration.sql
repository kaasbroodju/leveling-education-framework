-- CreateTable
CREATE TABLE "Skill" (
    "skill" TEXT NOT NULL,

    CONSTRAINT "Skill_pkey" PRIMARY KEY ("skill")
);

-- CreateTable
CREATE TABLE "SkillDescription" (
    "skillId" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "description" TEXT NOT NULL,
    "sublament" TEXT,

    CONSTRAINT "SkillDescription_pkey" PRIMARY KEY ("skillId","level")
);

-- CreateTable
CREATE TABLE "ArchitectureLayer" (
    "architectureLayer" TEXT NOT NULL,

    CONSTRAINT "ArchitectureLayer_pkey" PRIMARY KEY ("architectureLayer")
);

-- CreateTable
CREATE TABLE "Activity" (
    "activity" TEXT NOT NULL,

    CONSTRAINT "Activity_pkey" PRIMARY KEY ("activity")
);

-- CreateTable
CREATE TABLE "HBOIDescription" (
    "architectureLayerId" TEXT NOT NULL,
    "activityId" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "description" TEXT NOT NULL,
    "sublament" TEXT,

    CONSTRAINT "HBOIDescription_pkey" PRIMARY KEY ("architectureLayerId","activityId","level")
);

-- AddForeignKey
ALTER TABLE "SkillDescription" ADD CONSTRAINT "SkillDescription_skillId_fkey" FOREIGN KEY ("skillId") REFERENCES "Skill"("skill") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HBOIDescription" ADD CONSTRAINT "HBOIDescription_architectureLayerId_fkey" FOREIGN KEY ("architectureLayerId") REFERENCES "ArchitectureLayer"("architectureLayer") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HBOIDescription" ADD CONSTRAINT "HBOIDescription_activityId_fkey" FOREIGN KEY ("activityId") REFERENCES "Activity"("activity") ON DELETE RESTRICT ON UPDATE CASCADE;
