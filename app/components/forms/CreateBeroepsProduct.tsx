import { useState } from "react";
import {
	Button,
	Card,
	CardContent,
	CardHeader,
	FormControl,
	Grid2,
	IconButton,
	InputLabel,
	MenuItem,
	Modal,
	Select,
	SelectChangeEvent,
	Stack,
	TextField,
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import { activities } from "../../types/Activiteit";
import { architecture_layers } from "../../types/Architectuurlaag";
// import Form from "next/form";
import {
	createBeroepsproduct,
	CreateBeroepsproductDTO,
} from "../../lib/api/beroepsproducten";
import { guilds } from "../../types/Guild";

export function CreateBeroepsProduct() {
	const [open, setOpen] = useState(false);
	const [formData, setFormData] = useState<CreateBeroepsproductDTO>({
		title: "",
		layer: "",
		activity: "",
		guild: "",
	});

	const handleChange = (
		e:
			| React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
			| SelectChangeEvent,
	) => {
		setFormData({ ...formData, [e.target.name]: e.target.value });
	};

	const handleOpen = () => {
		setOpen(true);
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

		await createBeroepsproduct(formData);
	};

	return (
		<>
			<Grid2 size={12}>
				<Card component={"section"}>
					{/* TODO translate titles */}
					{/*<CardHeader title={props.title} component={"h1"}/>*/}
					<CardContent>
						<Button fullWidth={true} onClick={handleOpen} variant={"outlined"}>
							Create beroepsproduct
						</Button>
					</CardContent>
				</Card>
			</Grid2>
			<Modal open={open} onClose={handleClose}>
				<Card sx={style}>
					<CardHeader
						title={"Create beroepsproduct"}
						action={
							<IconButton onClick={handleClose}>
								<CloseIcon color={"primary"} />
							</IconButton>
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
									onChange={handleChange}
								/>
								<Grid2 container spacing={2}>
									<Grid2 size={5}>
										<FormControl fullWidth>
											<InputLabel id="layer-label">Architectuurlaag</InputLabel>
											<Select
												labelId="layer-label"
												id="layer-select"
												name="layer"
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
											<InputLabel id="activity-label">Activiteit</InputLabel>
											<Select
												labelId="activity-label"
												id="activity-select"
												name={"activity"}
												label="Activiteit"
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
						{/*<Box*/}
						{/*    component="form"*/}
						{/*    // sx={{ '& > :not(style)': { m: 1, width: '25ch' } }}*/}
						{/*    noValidate*/}
						{/*    autoComplete="off"*/}
						{/*    submit={createBeroepsProduct}*/}
						{/*>*/}

						{/*</Box>*/}
					</CardContent>
				</Card>
			</Modal>
		</>
	);
}
