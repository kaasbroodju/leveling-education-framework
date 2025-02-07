import { Card, CardContent, CardHeader, Grid } from "@mui/material";
import { ReactNode } from "react";

export function NavigationCard({
  title,
  subheader,
  children,
}: {
  title: string;
  subheader: string;
  children: ReactNode;
}) {
  return (
    <Card sx={{ minWidth: 300 }}>
      <CardContent aria-label={title} aria-description={subheader}>
        <Grid container spacing={1}>
          {children}
        </Grid>
      </CardContent>
    </Card>
  );
}
