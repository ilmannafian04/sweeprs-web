const util = {
    generateCode: (lenght=4): string => {
        let result = '';
        for (let i = 0; i < lenght; i++) {
            result += String.fromCharCode(Math.random() * (90 - 65) + 65);
            
        }
        return result;
    },
    addCodeToUrl: (code: string): string => {
        const url = new URL(window.location.href);
        url.searchParams.set('code', code);
        return url.href;
    }
}

export default util;
