import NextAuth from "next-auth";

declare module "next-auth" {
    interface User {
        role: string; // Add 'role' to the User type
    }

    interface Session {
        user: User; // Make sure Session has 'user' of type 'User'
    }
}