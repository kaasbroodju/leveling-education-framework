import {NextApiRequest, NextApiResponse} from "next";
import {db} from "../../../../lib/db";


export default async function handler(req: NextApiRequest, res: NextApiResponse) {
    switch (req.method) {
        case 'PUT':
            return updateBeroeopsProduct(req, res);
        case 'DELETE':
            return deleteBeroeopsProduct(req, res);
        default:
            return res.status(405).json({ message: 'Method Not Allowed' });
    }

}

async function updateBeroeopsProduct(req: NextApiRequest, res: NextApiResponse) {
    const { beroepsproduct } = req.query as { beroepsproduct: string }
    const { title, layer, activity, guild, level, sublament } = req.body
    const result = await db.hBOIExample.update({
        where: {
            id: beroepsproduct
        },
        data: {
            title,
            architectureLayerId: layer,
            activityId: activity,
            guild,
            level,
            sublament
        }})

    return res.status(200).json(result)
}

async function deleteBeroeopsProduct(req: NextApiRequest, res: NextApiResponse) {
    const { beroepsproduct } = req.query as { beroepsproduct: string }

    const deletedObj = await db.hBOIExample.delete({where: {id: beroepsproduct}});

    return res.status(200).json(deletedObj)
}