import React, { ReactNode } from "react";
import { useRouter } from "next/router";
import {Button, Grid, GridProps, useMediaQuery, useTheme} from "@mui/material";
import Link from "next/link";
import MenuIcon from "@mui/icons-material/Menu";
import {bgcolor, fontSize} from "@mui/system";
import {getIcon} from "../util/vaardighedenToIcon";
import {skills, TypeOfSkill} from "../types/Vaardigheid";

export function NavigationCardButtonSkill({
  title,
  query_param_key,
  query_param_value,
  type_of_skill,
  props = { xs: 12, sm: 6 },
}: {
  title: string | ReactNode;
  query_param_key: string;
  query_param_value: string;
  type_of_skill: TypeOfSkill | string;
  props?: GridProps;
}) {
  const router = useRouter();
  const theme = useTheme();
  theme.palette.primary
  const isLarge = useMediaQuery(theme.breakpoints.up('lg'));
  let href;
  if (router.query[query_param_key] === query_param_value) {
    const { [query_param_key]: _, ...query } = router.query;
    href = { query };
  } else {
    href = { query: { ...router.query, [query_param_key]: query_param_value } };
  }

  let activeColourMap = {
    "Beroeps": "rgba(152, 172, 204, 1)",
    "Persoonsvormende": "rgba(176, 196, 156, 1)",
    "Sociale": "rgba(232, 172, 140, 1)",
  }

  let colourMap = {
    "Beroeps": "rgba(152, 172, 204, .9)",
    "Persoonsvormende": "rgba(176, 196, 156, .9)",
    "Sociale": "rgba(232, 172, 140, .9)",
  }

  let colour = colourMap[type_of_skill];
  let activeColour = activeColourMap[type_of_skill];
  let isActive = query_param_value === router.query[query_param_key];

  let icon = getIcon(query_param_value);

  return (
    <Grid item {...props}>
      <Link href={href} style={{ width: "100%", color: theme.palette.primary.main }}>
        <Button
            sx={ isLarge ? { textTransform: "none", flexDirection: "column", aspectRatio: "1.7777777777777777", borderWidth: isActive ? 2 : 0, ":hover": { borderWidth: 2 } } : { textTransform: "none"}}
          variant={
            // isActive
            //   ? "contained"
            //   : "outlined"
              "outlined"
          }
          fullWidth
          style={{ backgroundColor: isActive ? activeColour : colour, color: theme.palette.primary.main }}
        >
          { isLarge
              ? icon
              : null
          }

          {title}
        </Button>
      </Link>
    </Grid>
  );
}
