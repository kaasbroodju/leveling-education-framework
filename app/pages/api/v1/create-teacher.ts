import { PrismaClient } from '@prisma/client';
import bcrypt from 'bcrypt';
import {NextApiRequest, NextApiResponse} from "next";

const prisma = new PrismaClient();

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
    if (req.method !== 'POST') return res.status(405).end();

    const { email, password } = req.body;

    // Check if a teacher already exists
    const existingTeacher = await prisma.user.findFirst({ where: { role: 'teacher' } });
    if (existingTeacher) {
        return res.status(400).json({ message: 'Teacher already exists' });
    }

    // Hash password and create user
    const hashedPassword = await bcrypt.hash(password, 10);
    await prisma.user.create({
        data: { email, password: hashedPassword, role: 'teacher' },
    });

    return res.status(201).json({ message: 'Teacher account created successfully' });
}
