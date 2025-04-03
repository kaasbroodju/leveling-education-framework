-- CreateTable
CREATE TABLE "HBOIExample" (
    "id" UUID NOT NULL,
    "architectureLayerId" TEXT NOT NULL,
    "activityId" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "guild" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "sublament" TEXT,

    CONSTRAINT "HBOIExample_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "HBOIExample" ADD CONSTRAINT "HBOIExample_architectureLayerId_fkey" FOREIGN KEY ("architectureLayerId") REFERENCES "ArchitectureLayer"("architectureLayer") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HBOIExample" ADD CONSTRAINT "HBOIExample_activityId_fkey" FOREIGN KEY ("activityId") REFERENCES "Activity"("activity") ON DELETE RESTRICT ON UPDATE CASCADE;
