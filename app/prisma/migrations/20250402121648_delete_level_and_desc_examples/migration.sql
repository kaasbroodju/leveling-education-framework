/*
  Warnings:

  - You are about to drop the column `level` on the `HBOIExample` table. All the data in the column will be lost.
  - You are about to drop the column `sublament` on the `HBOIExample` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "HBOIExample" DROP COLUMN "level",
DROP COLUMN "sublament";
