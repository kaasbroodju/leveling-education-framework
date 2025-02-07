import { createTheme } from "@mui/material/styles";

export const lightTheme = createTheme({
  palette: {
    mode: "light",
    primary: {
      main: "#333333",
    },
    background: {
      default: "#f0f0f0",
      paper: "#f5f5f5"
    },
    text: {
      primary: "#333333",
    },
  },
  components: {
    MuiAppBar: {
      styleOverrides: {
        colorPrimary: {
          color: "rgba(0,0,0,0.8)",
          backgroundColor: "#fff",
        },
      },
    },
  },
});
