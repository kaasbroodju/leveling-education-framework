import { useSession } from "next-auth/react";
import { useRouter } from "next/router";
import { JSX, useEffect } from "react";

export default function withAuth(Component: React.FC) {
	return function ProtectedComponent(props: JSX.IntrinsicAttributes) {
		const { data: session, status } = useSession();
		const router = useRouter();

		useEffect(() => {
			// @ts-expect-error undefined
			if (status === "unauthenticated" || session?.user?.role !== "teacher") {
				router.push("/login");
			}
		}, [status, session, router]);

		if (status === "loading") return <p>Loading...</p>;

		// @ts-expect-error undefined
		return session?.user?.role === "teacher" ? <Component {...props} /> : null;
	};
}
