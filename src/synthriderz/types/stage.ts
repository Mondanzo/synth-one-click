type StageFile = {
    id: number,
    filename: string,
    filename_original: string,
    extension: string,
    filesize: number,
    visiblity: "public",
    cdn_url: string,
    cdn_key: string,
    download_count: number,
    created_at: string
}

type BaseStageFile = {
    id: number,
    mode: string,
    platform: "pc" | "quest",
    description?: string,
    experience_tag?: string,
    created_at: string,
    file: StageFile
}

type BaseManifest = {
    isEXP: boolean,
    expTag: string,
    expDesc: string,
    fileName: string,
    available: boolean,
    isSpinStage: boolean,
    expbannerIcon: string,
    isNormalStage: boolean,
    normStageIcon: string,
    spinStageIcon: string
}


export type StageManifest = {
    exportedOn: string,
    creatorName: string,
    pc: BaseManifest,
    android: BaseManifest
};


export type StageFiles = {
    version: 1,
    manifest: StageManifest
} & BaseStageFile | {
    version: 2,
} & BaseStageFile


export type Stage = {
    cover_version: number,
    id: number,
    name: string,
    user: {
        id: number,
        username: string
    },
    files?: StageFiles[],
    latest_files: StageFiles[],
    download_url: string,
    cover_url: string,
}