import { Card, CardContent, CardHeader, Grid } from "@mui/material";
import { ReactNode } from "react";
import {TypeOfSkill} from "../types/Vaardigheid";

export function NavigationCardSkill({
  title,
  subheader,
  skills,
}: {
  title: string;
  subheader: string;
  skills: Record<TypeOfSkill, ReactNode[]>;
}) {
  return (
      <Card sx={{ minWidth: 300 }}>
        <CardContent aria-label={title} aria-description={subheader}>
          <Grid container spacing={1}>
            <Grid container item xs={12} lg={4.8}>
              <Grid container spacing={1}>
                <Grid item xs={12} lg={6}>
                  {skills["Beroeps"][0]}
                </Grid>
                <Grid item xs={12} lg={6}>
                  {skills["Beroeps"][1]}
                </Grid>
                <Grid item xs={12} lg={6}>
                  {skills["Beroeps"][2]}
                </Grid>
                <Grid item xs={12} lg={6}>
                  {skills["Beroeps"][3]}
                </Grid>
              </Grid>
            </Grid>
            <Grid container item xs={12} lg={7.2} spacing={1}>
              <Grid container item xs={12} spacing={1}>
                <Grid item xs={12} lg={4}>
                  {skills["Persoonsvormende"][0]}
                </Grid>
                <Grid item xs={12} lg={4}>
                  {skills["Persoonsvormende"][1]}
                </Grid>
                <Grid item xs={12} lg={4}>
                  {skills["Persoonsvormende"][2]}
                </Grid>
              </Grid>
              <Grid container item xs={12} spacing={1}>
                <Grid item xs={12} lg={4}>
                  {skills["Sociale"][0]}
                </Grid>
                <Grid item xs={12} lg={4}>
                  {skills["Sociale"][1]}
                </Grid>
                <Grid item xs={12} lg={4}>
                  {skills["Sociale"][2]}
                </Grid>
              </Grid>
            </Grid>
          </Grid>
        </CardContent>
      </Card>
  );
}
