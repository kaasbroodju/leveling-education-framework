import List from "@mui/material/List";
import React from "react";
import { Divider, Toolbar } from "@mui/material";
import { useRouter } from "next/router";
import { DrawerContentLink } from "./DrawerContentLink";

export default function DrawerContent(props: {
  handleDrawerClose: () => void;
}) {
  const { pathname } = useRouter();

  return (
    <>
      <Toolbar />
      <List>
        <DrawerContentLink
          onClick={props.handleDrawerClose}
          currentPathname={pathname}
          href={"/"}
          formattedMessageId={"SKILLS"}
        />
        <DrawerContentLink
          onClick={props.handleDrawerClose}
          currentPathname={pathname}
          href={"/beroepstaken"}
          formattedMessageId={"PROFESSIONAL_DUTIES"}
        />
        <DrawerContentLink
          onClick={props.handleDrawerClose}
          currentPathname={pathname}
          href={"/beroepsproducten"}
          formattedMessageId={"EXAMPLES_PROFESSIONAL_DUTIES"}
        />
        <DrawerContentLink
          onClick={props.handleDrawerClose}
          currentPathname={pathname}
          href={"/about"}
          formattedMessageId={"ABOUT"}
        />
      </List>
      <Divider />
    </>
  );
}
