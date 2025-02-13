import {getBeroepstakenOrVaardigheden} from "./getBeroepstakenOrVaardigheden";
import fsPromises from "fs/promises";
import {getBeroepstaken} from "./getBeroepstaken";

export async function migrateToNewFileLayout() {
    const lang: Array<("nl" | "en")> = ['nl', 'en']
    const typeOfSkill: Array<("hboi" | "vaardigheden")> = ['hboi', 'vaardigheden']

    for (let language of lang) {


        for (let type of typeOfSkill) {
            const source = await getBeroepstakenOrVaardigheden(type, language)

            for (let key of Object.keys(source)) {

                const taken = source[key]

                const activiteit = Object.keys(taken)


                for (let level of activiteit) {
                    console.log(key, level)
                    await fsPromises.mkdir(`datav2/${type}/${language}/${key}/${level}`, {recursive: true});
                    const taak = taken[level]
                    await fsPromises.writeFile(`datav2/${type}/${language}/${key}/${level}/description.txt`, taak.title)
                    if (taak.info !== "") {
                        await fsPromises.writeFile(`datav2/${type}/${language}/${key}/${level}/info.txt`, taak.info)

                    }

                    if (type === "hboi") {
                        await fsPromises.mkdir(`datav2/examples/${key}/${level}`, {recursive: true});
                        await fsPromises.writeFile(`datav2/examples/${key}/${level}.json`, "[]", 'utf-8')
                    }


                }

            }
        }


    }

    // const source = await getBeroepstakenOrVaardigheden("hboi", "nl")
    //
    // const architectuur = Object.keys(source)
    //
    //
    // for (let key of architectuur) {
    //     console.log(key)
    //     // await fsPromises.mkdir('tmp/' + key, {recursive: true});
    //     const taken = source[key]
    //
    //     const activiteit = Object.keys(taken)
    //
    //
    //     for (let keyactiviteit of activiteit) {
    //         console.log(key, keyactiviteit)
    //         await fsPromises.mkdir('tmp/' + key + '/' + keyactiviteit, {recursive: true});
    //         const taak = taken[keyactiviteit]
    //         await fsPromises.writeFile('tmp/' + key + '/' + keyactiviteit + '/' + 'description.txt', taak.title)
    //         if (taak.info !== "") {
    //             await fsPromises.writeFile('tmp/' + key + '/' + keyactiviteit + '/' + 'info.txt', taak.info)
    //         }
    //
    //
    //     }
    //
    // }
}