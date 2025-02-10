import { Card, CardContent, CardHeader, Grid } from "@mui/material";
import { BeroepstaakOrVaardigheid } from "../types/BeroepstakenOrVaardigheden";
import { Level } from "./Level";
import {Niveau} from "../types/Niveau";
import {BeroepsProduct} from "../types/BeroepsProduct";
import {BeroepsLevel} from "./BeroepsLevel";

export function LevelsCardBeroepsproduct(props: {
  title: string;
  item: { [key in Niveau]: BeroepsProduct[] };
}) {
  return (
    <Grid item xs={12}>
      <Card component={"section"}>
        {/* TODO translate titles */}
        <CardHeader title={props.title} component={"h1"}/>
        <CardContent>
          <Grid container spacing={5}>
            {Object.keys(props.item).map((niveauKey) => (
              <BeroepsLevel
                  niveauKey={niveauKey}
                products={props.item[niveauKey]}
                title={niveauKey}
              />
            ))}
          </Grid>
        </CardContent>
      </Card>
    </Grid>
  );
}
