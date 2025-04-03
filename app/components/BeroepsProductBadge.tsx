import {
	Button,
	Grid2,
	Stack,
	Typography,
	Modal,
	Card,
	CardHeader,
	CardContent,
	IconButton,
	TextField,
	FormControl,
	InputLabel,
	Select,
	MenuItem,
	SelectChangeEvent,
} from "@mui/material";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { useState } from "react";
import CloseIcon from "@mui/icons-material/Close";
import { architecture_layers } from "../types/Architectuurlaag";
import { activities } from "../types/Activiteit";
import {
	CreateBeroepsproductDTO,
	deleteBeroepsproduct,
	updateBeroepsproduct,
} from "../lib/api/beroepsproducten";
import { useSession } from "next-auth/react";
import { Delete } from "@mui/icons-material";
import { guildToColour } from "../util/guildToColour";
import { guilds } from "../types/Guild";

export function BeroepsProductBadge(props: { product: BeroepsProduct }) {
	const { data: session } = useSession();
	const isLoggedIn = !!session;

	const [open, setOpen] = useState(false);
	const [formData, setFormData] = useState<CreateBeroepsproductDTO>({
		title: props.product.title,
		layer: props.product.architectureLayerId,
		activity: props.product.activityId,
		guild: props.product.guild,
	});

	const handleChange = (
		e:
			| React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
			| SelectChangeEvent,
	) => {
		setFormData({ ...formData, [e.target.name]: e.target.value });
	};

	const handleOpen = () => {
		if (isLoggedIn) {
			setOpen(true);
		}
	};

	const handleDelete = async () => {
		if (
			window.confirm(`Are you sure you want to delete ${props.product.title}`)
		) {
			await deleteBeroepsproduct(props.product.id);
			setOpen(false);
		}
	};

	const handleClose = () => {
		setOpen(false);
	};

	const style = {
		position: "absolute",
		top: "50%",
		left: "50%",
		transform: "translate(-50%, -50%)",
		width: "50vw",
		// bgcolor: '#1f1f1f',
		// border: '2px solid #000',
		// boxShadow: 24,
		p: 4,
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();

		await updateBeroepsproduct(props.product.id, formData);
		setOpen(false);
	};

	const cardContent = isLoggedIn ? (
		<Card sx={style}>
			<CardHeader
				title={"Update beroepsproduct"}
				action={
					<Stack direction={"row"}>
						<IconButton onClick={handleDelete}>
							<Delete color={"error"} />
						</IconButton>
						<IconButton onClick={handleClose}>
							<CloseIcon color={"primary"} />
						</IconButton>
					</Stack>
				}
			></CardHeader>
			<CardContent>
				<form onSubmit={handleSubmit}>
					<Stack spacing={2}>
						<TextField
							id="outlined-basic"
							name={"title"}
							label="Title"
							variant="outlined"
							fullWidth
							defaultValue={props.product.title}
							onChange={handleChange}
						/>
						<Grid2 container spacing={2}>
							<Grid2 size={5}>
								<FormControl fullWidth>
									<InputLabel id="demo-simple-select-label">
										Architectuurlaag
									</InputLabel>
									<Select
										labelId="demo-simple-select-label"
										id="demo-simple-select"
										name="layer"
										defaultValue={props.product.architectureLayerId}
										label="Architectuurlaag"
										onChange={handleChange}
										// value={''}
									>
										{architecture_layers.map((layer) => (
											<MenuItem key={layer} value={layer}>
												{layer}
											</MenuItem>
										))}
									</Select>
								</FormControl>
							</Grid2>
							<Grid2 size={5}>
								<FormControl fullWidth>
									<InputLabel id="demo-simple-select-label">
										Activiteit
									</InputLabel>
									<Select
										labelId="demo-simple-select-label"
										id="demo-simple-select"
										name={"activity"}
										label="Activiteit"
										defaultValue={props.product.activityId}
										onChange={handleChange}
										// value={''}
									>
										{activities.map((activity) => (
											<MenuItem key={activity} value={activity}>
												{activity}
											</MenuItem>
										))}
									</Select>
								</FormControl>
							</Grid2>
							<Grid2 size={2}>
								<FormControl fullWidth>
									<InputLabel id="guild-label">Guild</InputLabel>
									<Select
										labelId="guild-label"
										id="guild-select"
										name={"guild"}
										label="Guild"
										defaultValue={props.product.guild}
										onChange={handleChange}
										// value={''}
									>
										{guilds.map((guild) => (
											<MenuItem key={guild} value={guild}>
												{guild}
											</MenuItem>
										))}
									</Select>
								</FormControl>
							</Grid2>
						</Grid2>
						<Button fullWidth variant={"contained"} type={"submit"}>
							Submit
						</Button>
					</Stack>
				</form>
			</CardContent>
		</Card>
	) : (
		<Card sx={style}>
			<CardContent>
				<Stack
					direction="row"
					justifyContent="space-between"
					alignItems="center"
				>
					<Typography variant="h5">{props.product.title}</Typography>
					<IconButton onClick={handleClose}>
						<CloseIcon color={"primary"} />
					</IconButton>
				</Stack>
			</CardContent>
		</Card>
	);

	return (
		<>
			<Button
				sx={{
					textAlign: "left",
					padding: "0",
					justifyContent: "left",
					textTransform: "none",
					alignItems: "baseline",
				}}
				fullWidth={true}
				onClick={handleOpen}
			>
				<Typography
					component={"span"}
					sx={{
						backgroundColor: guildToColour(props.product.guild),
						borderRadius: "4px",
						padding: "0 1ch",
						display: "inline-block",
						minWidth: "4ch",
						flex: "",
						textAlign: "center",
					}}
					variant="subtitle2"
					// whiteSpace="pre-wrap"
					// sx={{ wordBreak: "break-word" }}
				>
					{props.product.guild}
				</Typography>
				<Typography
					component={"span"}
					sx={{ paddingLeft: "1ch" }}
					variant="subtitle1"
					// whiteSpace="pre-wrap"
					// sx={{ wordBreak: "break-word" }}
				>
					{props.product.title}
				</Typography>
			</Button>
			<Modal open={open} onClose={handleClose}>
				{cardContent}
			</Modal>
		</>
	);
}
