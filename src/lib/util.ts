const basename = (path: string) => {
    return path.split('/').reverse()[0];
}

export { basename }
