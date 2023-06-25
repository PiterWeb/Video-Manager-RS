export default interface File {
    name: string;
    path: string;
    type: 'file' | 'dir';
    size?: number;
    lastModified?: number;
}