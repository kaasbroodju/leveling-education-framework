import { FormattedMessage, useIntl } from "react-intl";
import {
	Card,
	CardContent,
	IconButton,
	Modal,
	Stack,
	Tooltip,
	Typography,
} from "@mui/material";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import { useState } from "react";
import ReactMarkdown from "react-markdown";
import CloseIcon from "@mui/icons-material/Close";

export function InfoDrawerButton(props: { niveau: string; info: string }) {
	const [open, setOpen] = useState(false);
	const intl = useIntl();

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

	return (
		<>
			<Modal open={open} onClose={handleClose}>
				<Card sx={style}>
					<CardContent>
						<Stack
							direction="row"
							justifyContent="space-between"
							alignItems="center"
						>
							<Typography variant="h5">
								<FormattedMessage id="NIVEAU" /> {props.niveau}
							</Typography>
							<IconButton onClick={handleClose}>
								<CloseIcon color={"primary"} />
							</IconButton>
						</Stack>
						<Typography>
							<ReactMarkdown
								className="markdown"
								components={{
									h1: "h2",
								}}
							>
								{props.info}
							</ReactMarkdown>
						</Typography>
					</CardContent>
				</Card>
			</Modal>
			<Tooltip
				arrow
				title={intl.formatMessage(
					{
						id: "INFO_BUTTON_TOOLTIP",
					},
					{ niveau: props.niveau },
				)}
			>
				<IconButton onClick={handleOpen}>
					<InfoOutlinedIcon color={"primary"} />
				</IconButton>
			</Tooltip>
		</>
	);
}
