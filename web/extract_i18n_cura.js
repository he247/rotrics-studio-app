const fs = require('fs');

// Read from filePathIn and write normalized JSON to filePathOutput.
const extract = (filePathIn, filePathOutput) => {
    const jsObj = {};
    const content = fs.readFileSync(filePathIn, 'utf8');
    const arr = content.split("\n#: fdmprinter.def.json");
    for (let i = 0; i < arr.length; i++) {
        const item = arr[i].trim();
        const contents = item.split("\n");
        if (contents.length === 3) {
            const line0 = contents[0].trim();
            const line1 = contents[1].trim();
            const line2 = contents[2].trim();
            if (line0.indexOf("msgctxt ") === 0 &&
                line1.indexOf("msgid ") === 0 &&
                line2.indexOf("msgstr ") === 0
            ) {
                let msgid = line1.replace("msgid", "").trim();
                let msgstr = line2.replace("msgstr", "").trim();

                // Remove wrapping quotes and unescape backslashes.
                // JSON serialization will handle the final escaping format.
                msgid = msgid.substr(1, msgid.length - 2).replace(/\\/g, "").trim();
                msgstr = msgstr.substr(1, msgstr.length - 2).replace(/\\/g, "").trim();

                if (msgstr.length === 0) {
                    jsObj[msgid] = msgid;
                } else {
                    jsObj[msgid] = msgstr;
                }
            }
        }
    }
    fs.writeFileSync(filePathOutput, JSON.stringify(jsObj, null, 2));
};

/**
 * Extract translation keys from i18n_cura and write outputs to:
 * /build-web/asset/i18n/cura/
 */
const extract_i18n_cura = () => {
    const dirIn = "./i18n_cura/";
    const dirOutput = "./build-web/asset/i18n/cura/";
    const fdmJsonFilename = "fdmprinter.def.json.po";

    fs.mkdirSync(dirOutput, {recursive: true});

    const filenames = fs.readdirSync(dirIn);
    const subDirNames = []; // Folder names under i18n_cura.
    filenames.forEach((filename) => {
        const stats = fs.statSync(`${dirIn}${filename}`);
        if (stats.isDirectory()) {
            subDirNames.push(filename)
        }
    });

    subDirNames.forEach((subDirName) => {
        const filePathIn = `${dirIn}${subDirName}/${fdmJsonFilename}`;
        // Convert locale names from underscore to dash (e.g. zh_CN -> zh-CN).
        const filePathOut = `${dirOutput}${subDirName.replace("_", "-")}.json`;
        if (fs.existsSync(filePathIn)) {
            extract(filePathIn, filePathOut);
        }
    });

    // English template
    const filePathIn4en = `${dirIn}fdmprinter.def.json.pot`;
    const filePathOutput4en = `${dirOutput}en.json`;

    extract(filePathIn4en, filePathOutput4en);
};

module.exports = extract_i18n_cura;
