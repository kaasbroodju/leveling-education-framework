import {
	AccessibilityNew,
	Announcement,
	Biotech,
	Build,
	CalendarMonth,
	Chat,
	CoPresent,
	DesignServices,
	DirectionsRun,
	Domain,
	Explore,
	Handshake,
	Laptop,
	Memory,
	MenuBook,
	Psychology,
	SmartButton,
	Storage,
	Tune,
} from "@mui/icons-material";
import MenuIcon from "@mui/icons-material/Menu";
import React from "react";

export function getIcon(query_param_value: string, fontSize = 48) {
	switch (query_param_value) {
		case "Juiste kennis ontwikkelen":
			return <MenuBook sx={{ fontSize }} />;
		case "Kwalitatief product maken":
			return <Biotech sx={{ fontSize }} />;
		case "Overzicht creëren":
			return <Explore sx={{ fontSize }} />;
		case "Kritisch oordelen":
			return <Announcement sx={{ fontSize }} />;
		case "Samenwerken":
			return <Handshake sx={{ fontSize }} />;
		case "Boodschap delen":
			return <CoPresent sx={{ fontSize }} />;
		case "Plannen":
			return <CalendarMonth sx={{ fontSize }} />;
		case "Flexibel opstellen":
			return <AccessibilityNew sx={{ fontSize }} />;
		case "Pro-actief handelen":
			return <DirectionsRun sx={{ fontSize }} />;
		case "Reflecteren":
			return <Psychology sx={{ fontSize }} />;
		// architectuurlaag
		case "Gebruikersinteractie":
			return <SmartButton sx={{ fontSize }} />;
		case "Organisatieprocessen":
			return <Domain sx={{ fontSize }} />;
		case "Infrastructuur":
			return <Storage sx={{ fontSize }} />;
		case "Software":
			return <Laptop sx={{ fontSize }} />;
		case "Hardwareinterfacing":
			return <Memory sx={{ fontSize }} />;
		// activiteit
		case "Analyseren":
			return <Biotech sx={{ fontSize }} />;
		case "Adviseren":
			return <Chat sx={{ fontSize }} />;
		case "Ontwerpen":
			return <DesignServices sx={{ fontSize }} />;
		case "Realiseren":
			return <Build sx={{ fontSize }} />;
		case "Manage & Control":
			return <Tune sx={{ fontSize }} />;
		default:
			return <MenuIcon sx={{ fontSize }} />;
	}
}
