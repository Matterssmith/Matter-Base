#!/usr/bin/env node

"use strict";

/*
|--------------------------------------------------------------------------
| Base Bot Installer
|--------------------------------------------------------------------------
| Universal JavaScript Installer
| Works on:
|   • Windows
|   • Linux
|   • macOS
|   • Android (Termux)
|--------------------------------------------------------------------------
*/

import fs from "node:fs";
import fsp from "node:fs/promises";

import path from "node:path";
import os from "node:os";

import crypto from "node:crypto";
import https from "node:https";

import { pipeline } from "node:stream/promises";
import { spawn } from "node:child_process";

/*
|--------------------------------------------------------------------------
| Configuration
|--------------------------------------------------------------------------
*/

const CONFIG = {

    repository: "matterssmith/Matter-Base",

    branch: "matter",

    installer: "install-javascript.js",

    tempFolder: path.join(os.tmpdir(), "BaseBotInstaller"),

    preserve: [

        ".env",

        "runtime.state.json",

        "config.json"

    ]

};

/*
|--------------------------------------------------------------------------
| Colors
|--------------------------------------------------------------------------
*/

const COLOR = {

    reset: "\x1b[0m",

    red: "\x1b[31m",

    green: "\x1b[32m",

    yellow: "\x1b[33m",

    blue: "\x1b[34m",

    magenta: "\x1b[35m",

    cyan: "\x1b[36m",

    white: "\x1b[37m",

    gray: "\x1b[90m"

};

/*
|--------------------------------------------------------------------------
| Logger
|--------------------------------------------------------------------------
*/

class Logger {

    static line() {

        console.log();

    }

    static info(text) {

        console.log(`${COLOR.cyan}${text}${COLOR.reset}`);

    }

    static success(text) {

        console.log(`${COLOR.green}${text}${COLOR.reset}`);

    }

    static warning(text) {

        console.log(`${COLOR.yellow}${text}${COLOR.reset}`);

    }

    static error(text) {

        console.error(`${COLOR.red}${text}${COLOR.reset}`);

    }

}

/*
|--------------------------------------------------------------------------
| Sleep
|--------------------------------------------------------------------------
*/

function sleep(ms){

    return new Promise(resolve=>setTimeout(resolve,ms));

}

/*
|--------------------------------------------------------------------------
| Ensure Directory
|--------------------------------------------------------------------------
*/

async function ensureDirectory(directory){

    await fsp.mkdir(directory,{recursive:true});

}

/*
|--------------------------------------------------------------------------
| Remove Directory
|--------------------------------------------------------------------------
*/

async function removeDirectory(directory){

    await fsp.rm(directory,{

        recursive:true,

        force:true

    });

}

/*
|--------------------------------------------------------------------------
| File Exists
|--------------------------------------------------------------------------
*/

async function exists(file){

    try{

        await fsp.access(file);

        return true;

    }

    catch{

        return false;

    }

}

/*
|--------------------------------------------------------------------------
| SHA256
|--------------------------------------------------------------------------
*/

async function sha256(file){

    return new Promise((resolve,reject)=>{

        const hash=crypto.createHash("sha256");

        const stream=fs.createReadStream(file);

        stream.on("data",chunk=>hash.update(chunk));

        stream.on("end",()=>resolve(hash.digest("hex")));

        stream.on("error",reject);

    });

}

/*
|--------------------------------------------------------------------------
| Download File
|--------------------------------------------------------------------------
*/

async function download(url,destination){

    await ensureDirectory(path.dirname(destination));

    return new Promise((resolve,reject)=>{

        https.get(url,response=>{

            if(response.statusCode!==200){

                reject(new Error(`HTTP ${response.statusCode}`));

                return;

            }

            const file=fs.createWriteStream(destination);

            pipeline(response,file)

                .then(resolve)

                .catch(reject);

        }).on("error",reject);

    });

}

/*
|--------------------------------------------------------------------------
| Execute Command
|--------------------------------------------------------------------------
*/

function execute(command,args=[],cwd=process.cwd()){

    return new Promise((resolve,reject)=>{

        const child=spawn(command,args,{

            cwd,

            stdio:"inherit",

            shell:true

        });

        child.on("close",code=>{

            if(code===0)

                resolve();

            else

                reject(new Error(`${command} exited with ${code}`));

        });

    });

}

/*
|--------------------------------------------------------------------------
| Internet Connection
|--------------------------------------------------------------------------
*/

async function hasInternet() {

    return new Promise(resolve => {

        const request = https.request({

            method: "HEAD",

            host: "github.com",

            timeout: 5000

        }, () => resolve(true));

        request.on("error", () => resolve(false));

        request.on("timeout", () => {

            request.destroy();

            resolve(false);

        });

        request.end();

    });

}

/*
|--------------------------------------------------------------------------
| Download With Progress
|--------------------------------------------------------------------------
*/

async function downloadWithProgress(url, destination) {

    await ensureDirectory(path.dirname(destination));

    return new Promise((resolve, reject) => {

        https.get(url, response => {

            if (
                response.statusCode >= 300 &&
                response.statusCode < 400 &&
                response.headers.location
            ) {

                return downloadWithProgress(
                    response.headers.location,
                    destination
                )
                .then(resolve)
                .catch(reject);

            }


            if (response.statusCode !== 200) {

                reject(
                    new Error(
                        `Download failed (HTTP ${response.statusCode})`
                    )
                );

                return;

            }


            const total = Number(
                response.headers["content-length"] || 0
            );


            let downloaded = 0;


            const file = fs.createWriteStream(destination);


            response.on("data", chunk => {

                downloaded += chunk.length;


                if (total > 0) {

                    const percent = (
                        downloaded / total * 100
                    ).toFixed(1);


                    process.stdout.write(
                        `\rDownloading... ${percent}%`
                    );

                }

            });


            pipeline(response, file)

                .then(() => {

                    process.stdout.write("\n");

                    resolve();

                })

                .catch(reject);


        })

        .on("error", reject);

    });

}

/*
|--------------------------------------------------------------------------
| Download Binary File
|--------------------------------------------------------------------------
*/

async function downloadFile(url, destination) {

    await ensureDirectory(
        path.dirname(destination)
    );

    return new Promise((resolve, reject) => {

        https.get(url, {

            headers: {

                "User-Agent": "BaseBotInstaller"

            }

        }, response => {


            if (
                response.statusCode >= 300 &&
                response.statusCode < 400 &&
                response.headers.location
            ) {

                return downloadFile(
                    response.headers.location,
                    destination
                )
                .then(resolve)
                .catch(reject);

            }


            if (response.statusCode !== 200) {

                reject(
                    new Error(
                        `Download failed (HTTP ${response.statusCode})`
                    )
                );

                return;

            }


            const file = fs.createWriteStream(
                destination
            );


            pipeline(
                response,
                file
            )
            .then(resolve)
            .catch(reject);


        })

        .on("error", reject);

    });

}

/*
|--------------------------------------------------------------------------
| GitHub URLs
|--------------------------------------------------------------------------
*/

function getRawURL(file) {

    return `https://raw.githubusercontent.com/${CONFIG.repository}/${CONFIG.branch}/${file}`;

}

function getTreeURL() {

    return `https://api.github.com/repos/${CONFIG.repository}/git/trees/${CONFIG.branch}?recursive=1`;

}

function getContentURL(file) {

    return `https://raw.githubusercontent.com/${CONFIG.repository}/${CONFIG.branch}/${file}`;

}

/*
|--------------------------------------------------------------------------
| Compare Files
|--------------------------------------------------------------------------
*/

async function filesAreEqual(fileA, fileB) {

    if (!await exists(fileA))

        return false;

    if (!await exists(fileB))

        return false;

    return (await sha256(fileA)) === (await sha256(fileB));

}

/*
|--------------------------------------------------------------------------
| Update Installer
|--------------------------------------------------------------------------
*/

async function updateInstaller() {

    Logger.info("Checking installer updates...");

    const online = await hasInternet();

    if (!online) {

        Logger.error("No Internet connection.");

        process.exit(1);

    }

    await ensureDirectory(CONFIG.tempFolder);

    const remoteInstaller = path.join(

        CONFIG.tempFolder,

        CONFIG.installer

    );

    await downloadWithProgress(

        getRawURL(CONFIG.installer),

        remoteInstaller

    );

    const currentInstaller = path.resolve(

        CONFIG.installer

    );

    const equal = await filesAreEqual(

        currentInstaller,

        remoteInstaller

    );

    if (equal) {

        Logger.success("Installer is already up to date.");

        return false;

    }

    Logger.warning("New installer version found.");

    await fsp.copyFile(

        remoteInstaller,

        currentInstaller

    );

    Logger.success("Installer updated.");

    return true;

}

/*
|--------------------------------------------------------------------------
| Restart Installer
|--------------------------------------------------------------------------
*/

async function restartInstaller() {

    Logger.info("Restarting installer...");

    spawn(

        process.execPath,

        [path.resolve(CONFIG.installer)],

        {

            detached: true,

            stdio: "inherit"

        }

    );

    process.exit(0);

}

/*
|--------------------------------------------------------------------------
| Temporary Paths
|--------------------------------------------------------------------------
*/

CONFIG.treeFile = path.join(
    CONFIG.tempFolder,
    "tree.json"
);

CONFIG.downloadFolder = path.join(
    CONFIG.tempFolder,
    "repository"
);

/*
|--------------------------------------------------------------------------
| Download Repository Tree
|--------------------------------------------------------------------------
*/

async function downloadRepository() {

    Logger.info("Downloading repository tree...");

    await ensureDirectory(CONFIG.tempFolder);

    const response = await fetch(getTreeURL(), {
        headers: {
            "User-Agent": "BaseBotInstaller"
        }
    });

    if (!response.ok) {

        throw new Error(
            `GitHub API returned HTTP ${response.status}`
        );

    }

    const tree = await response.json();

    await fsp.writeFile(
        CONFIG.treeFile,
        JSON.stringify(tree, null, 2),
        "utf8"
    );

    Logger.success("Repository tree downloaded.");

}

/*
|--------------------------------------------------------------------------
| Download Repository Files
|--------------------------------------------------------------------------
*/

async function downloadRepositoryFiles() {

    Logger.info(
        "Downloading repository files..."
    );


    const tree = JSON.parse(

        await fsp.readFile(

            CONFIG.treeFile,

            "utf8"

        )

    );


    await ensureDirectory(

        CONFIG.downloadFolder

    );


    for (const item of tree.tree) {


        if (item.type !== "blob")

            continue;


        const destination = path.join(

            CONFIG.downloadFolder,

            item.path

        );


        Logger.info(

            "Downloading: " + item.path

        );


        await downloadFile(

            getContentURL(item.path),

            destination

        );


    }


    Logger.success(

        "Repository files downloaded."

    );

}

/*
|--------------------------------------------------------------------------
| Remove Temporary Files
|--------------------------------------------------------------------------
*/

async function removeTemporaryFiles() {

    if (await exists(CONFIG.treeFile)) {

        await fsp.unlink(
            CONFIG.treeFile
        );

    }

}

/*
|--------------------------------------------------------------------------
| Ignored Files
|--------------------------------------------------------------------------
*/

function isIgnored(relativePath) {

    const normalized = relativePath.replace(/\\/g, "/");

    return CONFIG.preserve.some(file =>

        normalized === file ||

        normalized.endsWith("/" + file)

    );

}

/*
|--------------------------------------------------------------------------
| Copy File
|--------------------------------------------------------------------------
*/

async function copyFile(source, destination) {

    await ensureDirectory(path.dirname(destination));

    await fsp.copyFile(source, destination);

}

/*
|--------------------------------------------------------------------------
| Synchronize File
|--------------------------------------------------------------------------
*/

async function synchronizeFile(source, destination) {

    if (!(await exists(destination))) {

        await copyFile(source, destination);

        Logger.success("Added: " + path.relative(process.cwd(), destination));

        return;

    }

    const sourceHash = await sha256(source);

    const destinationHash = await sha256(destination);

    if (sourceHash === destinationHash)

        return;

    await copyFile(source, destination);

    Logger.info("Updated: " + path.relative(process.cwd(), destination));

}

/*
|--------------------------------------------------------------------------
| Synchronize Directory
|--------------------------------------------------------------------------
*/

async function synchronizeDirectory(source, destination) {

    const entries = await fsp.readdir(source, {

        withFileTypes: true

    });

    for (const entry of entries) {

        const sourcePath = path.join(source, entry.name);

        const destinationPath = path.join(destination, entry.name);

        const relative = path.relative(process.cwd(), destinationPath);

        if (isIgnored(relative))

            continue;

        if (entry.isDirectory()) {

            await synchronizeDirectory(

                sourcePath,

                destinationPath

            );

            continue;

        }

        await synchronizeFile(

            sourcePath,

            destinationPath

        );

    }

}

/*
|--------------------------------------------------------------------------
| Repository Synchronization
|--------------------------------------------------------------------------
*/

async function synchronizeRepository() {

    Logger.info("Synchronizing repository...");

    await synchronizeDirectory(

        CONFIG.downloadFolder,

        process.cwd()

    );

    Logger.success("Repository synchronized.");

}

/*
|--------------------------------------------------------------------------
| Delete Obsolete Files
|--------------------------------------------------------------------------
*/

async function removeObsolete(localDir, repoDir) {

    const entries = await fsp.readdir(localDir, {

        withFileTypes: true

    });

    for (const entry of entries) {

        const localPath = path.join(localDir, entry.name);

        const repoPath = path.join(repoDir, entry.name);

        const relative = path.relative(

            process.cwd(),

            localPath

        );

        if (isIgnored(relative))

            continue;

        if (!(await exists(repoPath))) {

            if (entry.isDirectory()) {

                await removeDirectory(localPath);

            }

            else {

                await fsp.unlink(localPath);

            }

            Logger.warning("Removed: " + relative);

            continue;

        }

        if (

            entry.isDirectory()

        ) {

            await removeObsolete(

                localPath,

                repoPath

            );

        }

    }

}


/*
|--------------------------------------------------------------------------
| Clean Repository
|--------------------------------------------------------------------------
*/

async function cleanRepository() {

    Logger.info("Removing obsolete files...");

    await removeObsolete(

        process.cwd(),

        CONFIG.downloadFolder

    );

    Logger.success("Repository cleaned.");

}

/*
|--------------------------------------------------------------------------
| Prepare Temporary Directory
|--------------------------------------------------------------------------
*/

async function prepareTemp() {

    if (await exists(CONFIG.tempFolder)) {

        await removeDirectory(
            CONFIG.tempFolder
        );

    }

    await ensureDirectory(
        CONFIG.tempFolder
    );

}

/*
|--------------------------------------------------------------------------
| Synchronize Repository
|--------------------------------------------------------------------------
*/

async function synchronize() {

    await prepareTemp();

    await downloadRepository();

    await downloadRepositoryFiles();

    await synchronizeRepository();

    await cleanRepository();

}

/*
|--------------------------------------------------------------------------
| Cleanup
|--------------------------------------------------------------------------
*/

async function cleanup() {

    if (

        await exists(CONFIG.tempFolder)

    ) {

        await removeDirectory(CONFIG.tempFolder);

    }

}

/*
|--------------------------------------------------------------------------
| Fatal Error
|--------------------------------------------------------------------------
*/

async function fatal(error) {

    Logger.line();

    Logger.error(error.message);

    Logger.line();

    await cleanup();

    process.exit(1);

}

/*
|--------------------------------------------------------------------------
| NPM Package Installation
|--------------------------------------------------------------------------
*/

async function installDependencies() {

    Logger.info("Installing npm dependencies...");


    const packageFile = path.join(

        process.cwd(),

        "package.json"

    );


    if (!await exists(packageFile)) {

        Logger.warning("package.json not found. Skipping npm install.");

        return;

    }


    await execute(

        "npm",

        [

            "install",

            "--prefer-online"

        ],

        process.cwd()

    );


    Logger.success("npm dependencies installed.");

}

/*
|--------------------------------------------------------------------------
| Apply Patches
|--------------------------------------------------------------------------
*/

async function applyPatches() {

    Logger.info("Applying patches...");


    const patchFile = path.join(

        process.cwd(),

        "scripts",

        "patches.js"

    );


    if (!await exists(patchFile)) {

        Logger.warning("No patch script found. Skipping.");

        return;

    }


    await execute(

        process.execPath,

        [

            patchFile

        ],

        process.cwd()

    );


    Logger.success("Patches applied.");

}

/*
|--------------------------------------------------------------------------
| Start Bot
|--------------------------------------------------------------------------
*/

async function startBot() {

    Logger.info("Starting Base Bot...");


    const packageFile = path.join(

        process.cwd(),

        "package.json"

    );


    if (!await exists(packageFile)) {

        throw new Error("package.json not found. Cannot start bot.");

    }


    await execute(

        "npm",

        [

            "start"

        ],

        process.cwd()

    );


    Logger.success("Bot stopped.");

}

/*
|--------------------------------------------------------------------------
| Main Installer
|--------------------------------------------------------------------------
*/

async function main() {

    Logger.line();

    Logger.info("Base Bot Universal Installer");

    Logger.line();


    const updated = await updateInstaller();


    if (updated) {

        await restartInstaller();

        return;

    }


    await synchronize();


    await installDependencies();


    await applyPatches();


    await startBot();

}

/*
|--------------------------------------------------------------------------
| Installer Entry Point
|--------------------------------------------------------------------------
*/

main()

    .catch(fatal);
