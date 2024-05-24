import UAParser from "ua-parser-js";

/**
 * 根据用户的系统拼接 URL
 * 
 * @param {string} src - 原始的 URL
 * @param {string} path - 需要追加的路径
 * 
 * @returns {string} - 拼接后的 URL
 */
function appendPath(src: string, path: string): string {
    const parser = new UAParser();
    const system = parser.getOS().name;

    if (system === "Linux") {
        return src + "/" + path;
    } else if (system === "Windows") {
        return src + "\\" + path;
    } else {
        return src + "/" + path;
    }
}

export default appendPath;
