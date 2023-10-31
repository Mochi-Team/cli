export default async function handleServe(src?: string, dest?: string, site: boolean = false) {
    if (src == undefined) {
        src = process.cwd();
    }

    if (dest == undefined) {
        dest = src + '/dist';
    }

    console.log("serve w/ source: " + src + ', dest: ' + dest);
}