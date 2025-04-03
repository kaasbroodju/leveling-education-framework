import NextAuth, { NextAuthOptions } from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { PrismaClient } from "@prisma/client";
import bcrypt from "bcrypt";

type User = {
	id: string;
	email: string;
	password: string;
	role: string;
};

const prisma = new PrismaClient();

export const authOptions: NextAuthOptions = {
	providers: [
		CredentialsProvider({
			name: "credentials",
			credentials: {
				email: { label: "Email", type: "email" },
				password: { label: "Password", type: "password" },
			},
			async authorize(credentials) {
				const { email, password } = credentials as {
					email: string;
					password: string;
				};

				// Fetch user from database
				const user: User | null = await prisma.user.findUnique({
					where: { email },
				});

				if (!user || !(await bcrypt.compare(password, user.password))) {
					throw new Error("Invalid credentials");
				}

				return { id: user.id, email: user.email, role: user.role };
			},
		}),
	],
	callbacks: {
		async session({ session, token }) {
			// @ts-expect-error undefined
			session.user.role = token.role;
			return session;
		},
		async jwt({ token, user }) {
			// @ts-expect-error undefined
			if (user) token.role = user.role;
			return token;
		},
	},
	pages: {
		signIn: "/login",
	},
	secret: process.env.NEXTAUTH_SECRET,
};
export default NextAuth(authOptions);
