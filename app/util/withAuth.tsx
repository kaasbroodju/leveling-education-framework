import { useSession } from "next-auth/react";
import { useRouter } from "next/router";
import { useEffect } from "react";

export default function withAuth(Component: React.FC) {
  return function ProtectedComponent(props: any) {
    const { data: session, status } = useSession();
    const router = useRouter();

    useEffect(() => {
      if (status === "unauthenticated" || session?.user?.role !== "teacher") {
        router.push("/login");
      }
    }, [status, session, router]);

    if (status === "loading") return <p>Loading...</p>;

    return session?.user?.role === "teacher" ? <Component {...props} /> : null;
  };
}
