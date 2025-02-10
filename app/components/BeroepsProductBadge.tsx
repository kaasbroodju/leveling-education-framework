import {Box, Button, ButtonBase, Grid, Stack, Typography, Modal, Card, CardHeader, CardContent} from "@mui/material";
import {FormattedMessage, useIntl} from "react-intl";
import { InfoDrawerButton } from "./InfoDrawerButton";
import {BeroepsProduct} from "../types/BeroepsProduct";
import {useState} from "react";
import ReactMarkdown from "react-markdown";

export function BeroepsProductBadge(props: {
    product: BeroepsProduct
}) {
    const [open, setOpen] = useState(false);
    const intl = useIntl();

    const handleOpen = () => {
        setOpen(true);
    };

    const handleClose = () => {
        setOpen(false);
    };

    const style = {
        position: 'absolute',
        top: '50%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        width: '50vw',
        // bgcolor: '#1f1f1f',
        // border: '2px solid #000',
        // boxShadow: 24,
        p: 4,
    };

    return (
        <Box>
            <Button
                sx={{textAlign: 'left', padding: '0', justifyContent: 'left', textTransform: 'none', alignItems: 'baseline'}}
                fullWidth={true}
                onClick={handleOpen}
            >
                <Typography
                    component={"span"}
                    sx={{backgroundColor: "red", borderRadius: "4px", padding: "0 1ch", display: "inline-block", flex: ''}}
                    variant="subtitle2"
                    // whiteSpace="pre-wrap"
                    // sx={{ wordBreak: "break-word" }}
                >
                    {props.product.guild}
                </Typography>
                <Typography
                    component={"span"}
                    sx={{paddingLeft: "1ch"}}
                    variant="subtitle1"
                    // whiteSpace="pre-wrap"
                    // sx={{ wordBreak: "break-word" }}
                >
                    {props.product.product}
                </Typography>
            </Button>
            {/*<Button onClick={handleOpen}>Open modal</Button>*/}
            {props.product.desc !== undefined ?
                <Modal
                    open={open}
                    onClose={handleClose}
                    // aria-labelledby="modal-modal-title"
                    // aria-describedby="modal-modal-description"
                >
                    <Card sx={style}>
                        <CardHeader title={props.product.product} component={"h1"}>
                            {/*<Typography id="modal-modal-title" variant="h6" component="h2">*/}
                            {/*    {props.product.product}*/}
                            {/*</Typography>*/}
                        </CardHeader>
                        <CardContent>
                            <Typography>
                                <ReactMarkdown
                                    className="markdown"
                                    components={{
                                        h1: "h2",
                                    }}
                                >
                                    {props.product.desc}
                                </ReactMarkdown>
                            </Typography>
                        </CardContent>
                    </Card>
                </Modal> : null}
        </Box>
    );
}
