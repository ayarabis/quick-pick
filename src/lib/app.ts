import '$lib/app.postcss';
import '@mdi/font/css/materialdesignicons.min.css';
import '@skeletonlabs/skeleton/styles/all.css';
import '@skeletonlabs/skeleton/themes/theme-rocket.css';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import { Command } from '@tauri-apps/api/shell';
import { writable } from 'svelte/store';
import { Store } from 'tauri-plugin-store-api';

type ItemType = 'Folder' | 'File' | 'WebURL' | 'Snippet' | 'Action' | 'Note';

type Item = {
    id?: string
    name: string
    type: ItemType
    [key: string]: any
}

type Action = {
    name: string
    icon: string
    callback: () => void
}

type QuickApp = { app: string; iconName: string; iconPath: string };

const store = new Store('.items.dat');
const appStore = new Store('.apps.dat');
const journalStore = new Store('.journal.dat')
const canHide = writable(true);

export async function showNotification(title: string, body: string) {
    if (await isPermissionGranted()) {
        sendNotification({
            icon: 'icon.png',
            title,
            body
        });
    } else {
        await requestPermission();
    }
}

export async function openResource(item: Item) {
    const cmd = new Command('open', `${item.path}`, { cwd: '/' });
    cmd.on('error', (err) => {
        showNotification('Error', err);
    });
    cmd.stderr.on('data', (err) => {
        showNotification('Error', err);
    });
    await cmd.execute();
}

export async function execCommand(cmd: Command) {
    cmd.on('error', (err) => {
        console.log(err);
        showNotification('Error', err);
    });
    cmd.stderr.on('data', (err) => {
        console.log(err);
        showNotification('Error', err);
    });
    await cmd.execute();
}

export const extensionMap: { [key: string]: string } = {
    'jpg': 'mdi-image',
    'png': 'mdi-image',
    'pdf': 'mdi-file-pdf-box',
    'xlsx': 'mdi-file-excel',
    'csv': 'mdi-file-delimited',
    'json': 'mdi-code-json',
    'zip': 'mdi-folder-zip',
    'jar': 'mdi-zip-box',
    'pub': 'mdi-file',
    'doc': 'mdi-file-document-edit',
    'docx': 'mdi-file-document-edit',
    'pptx': 'mdi-file-powerpoint',
    'txt': 'mdi-note-text',
    'sql': 'mdi-database-search',
    'xml': 'mdi-xml',
    'dwl': 'mdi-code-braces',
    '': 'mdi-file-question'
}

export type { Item, Action, ItemType, QuickApp };
export { store, appStore, journalStore, canHide };

