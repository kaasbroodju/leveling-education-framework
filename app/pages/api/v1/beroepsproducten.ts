import {NextApiRequest, NextApiResponse} from "next";
import {db} from "../../../lib/db";

export default async function handler( req: NextApiRequest, res: NextApiResponse) {
    switch (req.method) {
        case 'POST':
            return createBeroepsproduct(req, res);
        case 'GET':
            return getBeroepsproducten(req, res);
        default:
            return res.status(405).json({ message: 'Method Not Allowed' });
    }

}

async function createBeroepsproduct( req: NextApiRequest, res: NextApiResponse) {
    console.log(req.body)
    const { title, layer, activity, guild, level, sublament } = req.body


    const example = await db.hBOIExample.create({ data: {
        title,
        architectureLayerId: layer,
        activityId: activity,
        guild,
        level,
        sublament
    }})

    return res.status(200).json(example)
}

async function getBeroepsproducten( req: NextApiRequest, res: NextApiResponse ) {

}