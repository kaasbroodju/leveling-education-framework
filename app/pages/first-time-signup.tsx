import { useState } from "react";
import {
	Alert,
	Button,
	Card,
	CardContent,
	CardHeader,
	Stack,
	TextField,
} from "@mui/material";

export default function FirstTimeSetup() {
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");
	const [error, setError] = useState("");
	const [message, setMessage] = useState("");

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();

		if (!email || !password) {
			setError("Please enter both email and password");
			return;
		}

		const res = await fetch("/api/v1/create-teacher", {
			method: "POST",
			body: JSON.stringify({ email, password }),
			headers: { "Content-Type": "application/json" },
		});

		const data = await res.json();
		if (res.ok) {
			setMessage(data.message);
		} else {
			setError(data.message);
		}
	};

	return (
		<Card>
			<CardHeader title={"First time signup"} />
			<CardContent>
				<Stack component="form" onSubmit={handleSubmit} spacing={4}>
					{error && <Alert severity="error">{error}</Alert>}
					{message && <Alert severity={"success"}>{message}</Alert>}
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
						Create account
					</Button>
				</Stack>
			</CardContent>
		</Card>
	);
}
