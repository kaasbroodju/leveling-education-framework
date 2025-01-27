import { Card, CardContent, CardHeader, Grid } from "@mui/material";
import { ReactNode } from "react";
import {TypeOfSkill} from "../types/Vaardigheid";

export function NavigationCardSkill({
  title,
  subheader,
  test,
  children,
}: {
  title: string | ReactNode;
  subheader: string | ReactNode;
  test: Record<TypeOfSkill, ReactNode[]>
  children: ReactNode;
}) {
  return (
      <Card sx={{ minWidth: 300 }}>
        <CardContent>
          <Grid container spacing={1}>
            {/* Subgrid for "a" */}
            {/* a a
                a a
            */}
            <Grid container item xs={4.8}>
              {/* a a */}
              <Grid container spacing={1}>
                {/* a */}
                <Grid item xs={6}>
                  {test["Beroeps"][0]}
                </Grid>
                <Grid item xs={6}>
                  {test["Beroeps"][1]}
                </Grid>
                <Grid item xs={6}>
                  {test["Beroeps"][2]}
                </Grid>
                <Grid item xs={6}>
                  {test["Beroeps"][3]}
                </Grid>
              </Grid>
              {/* a a */}
              {/*<Grid container item xs={12}>*/}
              {/*  <Grid item xs={6}>*/}
              {/*    {test["Beroeps"][2]}*/}
              {/*  </Grid>*/}
              {/*  <Grid item xs={6}>*/}
              {/*    {test["Beroeps"][3]}*/}
              {/*  </Grid>*/}
              {/*</Grid>*/}
            </Grid>


            <Grid container item xs={7.2}  spacing={1}>
              <Grid container item xs={12} spacing={1}>
                <Grid item xs={4}>
                  {test["Persoonsvormende"][0]}
                </Grid>
                <Grid item xs={4}>
                  {test["Persoonsvormende"][1]}
                </Grid>
                <Grid item xs={4}>
                  {test["Persoonsvormende"][2]}
                </Grid>
              </Grid>
              <Grid container item xs={12} spacing={1}>
                <Grid item xs={4}>
                  {test["Sociale"][0]}
                </Grid>
                <Grid item xs={4}>
                  {test["Sociale"][1]}
                </Grid>
                <Grid item xs={4}>
                  {test["Sociale"][2]}
                </Grid>
              </Grid>
            </Grid>
          </Grid>
        </CardContent>
      </Card>
  );
}
