export default interface Notification {
    type: NotificationType;
    title: string;
    msg: string;
}

export enum NotificationType {
    Err,
    Warn,
    Info,
}
