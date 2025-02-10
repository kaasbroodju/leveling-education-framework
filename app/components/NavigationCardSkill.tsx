import { Card, CardContent, CardHeader, Grid } from "@mui/material";
import { ReactNode } from "react";
import {Skill, TypeOfSkill} from "../types/Vaardigheid";

export function NavigationCardSkill({
  title,
  subheader,
  skills,
}: {
  title: string;
  subheader: string;
  skills: Record<TypeOfSkill, [Skill, ReactNode][]>
}) {
  return (
      <Card sx={{ minWidth: 300 }}>
        <CardContent aria-label={title} aria-description={subheader}>
          <Grid container spacing={1}>
            <Grid container item xs={12} lg={4.8}>
              <Grid container spacing={1}>
                {skills["Beroeps"].map(([skill, node]) =>
                    <Grid key={skill} item xs={12} lg={6}>
                      {node}
                    </Grid>
                )}
              </Grid>
            </Grid>
            <Grid container item xs={12} lg={7.2} spacing={1}>
              <Grid container item xs={12} spacing={1}>
                {skills["Persoonsvormende"].map(([skill, node]) =>
                    <Grid key={skill} item xs={12} lg={4}>
                      {node}
                    </Grid>
                )}
              </Grid>
              <Grid container item xs={12} spacing={1}>
                {skills["Sociale"].map(([skill, node]) =>
                    <Grid key={skill} item xs={12} lg={4}>
                      {node}
                    </Grid>
                )}
              </Grid>
            </Grid>
          </Grid>
        </CardContent>
      </Card>
  );
}
