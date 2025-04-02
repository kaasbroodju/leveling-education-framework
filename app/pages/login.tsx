import { useState } from "react";
import { signIn } from "next-auth/react";
import {
	Button,
	Card,
	CardContent,
	CardHeader,
	TextField,
	Alert,
	Stack,
} from "@mui/material";

export default function LoginPage() {
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");
	const [error, setError] = useState("");

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();

		if (!email || !password) {
			setError("Please enter both email and password");
			return;
		}

		const res = await signIn("credentials", {
			redirect: false,
			email,
			password,
		});

		if (res?.error) {
			setError("Invalid email or password.");
		} else {
			window.location.href = "/login"; // Redirect on success
		}
	};

	return (
		<Card>
			<CardHeader title={"Login"} />
			<CardContent>
				<Stack component="form" onSubmit={handleSubmit} spacing={4}>
					{error && <Alert severity="error">{error}</Alert>}
					<TextField
						margin="normal"
						required
						fullWidth
						id="email"
						label="Email Address"
						name="email"
						autoComplete="email"
						autoFocus
						variant="outlined"
						value={email}
						onChange={(e) => setEmail(e.target.value)}
					/>
					<TextField
						margin="normal"
						required
						fullWidth
						name="password"
						label="Password"
						type="password"
						id="password"
						autoComplete="current-password"
						variant="outlined"
						value={password}
						onChange={(e) => setPassword(e.target.value)}
					/>
					<Button
						type="submit"
						fullWidth
						variant="contained"
						// sx={{ mt: 3, mb: 2, py: 1.5 }}
					>
						Log In
					</Button>
				</Stack>
			</CardContent>
		</Card>
	);
}
