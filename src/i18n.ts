// 多语言支持模块
export type Language = 'zh-CN' | 'en-US' | 'ja-JP' | 'ko-KR';

export interface Translations {
  // 应用标题
  appTitle: string;
  appSubtitle: string;
  
  // 设备信息
  deviceInfo: string;
  deviceName: string;
  deviceType: string;
  ipAddress: string;
  
  // 文件操作
  selectFiles: string;
  selectedFiles: string;
  noFilesSelected: string;
  dragFilesHere: string;
  dragFilesHint: string;
  orClickButton: string;
  clearFiles: string;
  removeFile: string;
  
  // 设备扫描
  scanDevices: string;
  scanning: string;
  stopScan: string;
  devicesFound: string;
  noDevicesFound: string;
  refreshDevices: string;
  
  // 传输操作
  sendFiles: string;
  sendTo: string;
  transferProgress: string;
  transferComplete: string;
  transferFailed: string;
  transferCancelled: string;
  
  // 状态
  online: string;
  offline: string;
  connecting: string;
  transferring: string;
  paused: string;
  
  // 设置
  settings: string;
  darkMode: string;
  lightMode: string;
  language: string;
  autoStart: string;
  encryption: string;
  notifications: string;
  
  // 托盘菜单
  showWindow: string;
  hideWindow: string;
  quit: string;
  transferHistory: string;
  
  // 通知
  minimizedToTray: string;
  fileReceived: string;
  transferStarted: string;
  
  // 错误信息
  errorSelectFiles: string;
  errorSendFiles: string;
  errorConnection: string;
  errorFileAccess: string;
  
  // 安全功能
  secureTransfer: string;
  enterPassword: string;
  generateCode: string;
  securityCode: string;
  
  // 高级功能
  folderTransfer: string;
  pauseTransfer: string;
  resumeTransfer: string;
  cancelTransfer: string;
  bandwidthLimit: string;
  transferQueue: string;
}

// 中文翻译
const zhCN: Translations = {
  appTitle: '局域网传输工具',
  appSubtitle: '快速安全的文件传输',
  
  deviceInfo: '设备信息',
  deviceName: '设备名称',
  deviceType: '设备类型',
  ipAddress: 'IP地址',
  
  selectFiles: '选择文件',
  selectedFiles: '已选择文件',
  noFilesSelected: '未选择文件',
  dragFilesHere: '拖拽文件到此处',
  dragFilesHint: '松开鼠标即可添加文件',
  orClickButton: '或者点击下方按钮选择文件',
  clearFiles: '清空文件',
  removeFile: '移除',
  
  scanDevices: '扫描设备',
  scanning: '扫描中...',
  stopScan: '停止扫描',
  devicesFound: '发现设备',
  noDevicesFound: '未发现其他设备',
  refreshDevices: '刷新设备列表',
  
  sendFiles: '发送文件',
  sendTo: '发送到',
  transferProgress: '传输进度',
  transferComplete: '传输完成',
  transferFailed: '传输失败',
  transferCancelled: '传输已取消',
  
  online: '在线',
  offline: '离线',
  connecting: '连接中',
  transferring: '传输中',
  paused: '已暂停',
  
  settings: '设置',
  darkMode: '深色主题',
  lightMode: '浅色主题',
  language: '语言',
  autoStart: '开机启动',
  encryption: '加密传输',
  notifications: '桌面通知',
  
  showWindow: '显示窗口',
  hideWindow: '隐藏窗口',
  quit: '退出',
  transferHistory: '传输历史',
  
  minimizedToTray: '应用已最小化到系统托盘',
  fileReceived: '文件接收完成',
  transferStarted: '文件传输已开始',
  
  errorSelectFiles: '请先选择要发送的文件',
  errorSendFiles: '发送文件失败',
  errorConnection: '连接失败',
  errorFileAccess: '文件访问错误',
  
  secureTransfer: '安全传输',
  enterPassword: '请输入密码',
  generateCode: '生成验证码',
  securityCode: '安全验证码',
  
  folderTransfer: '文件夹传输',
  pauseTransfer: '暂停传输',
  resumeTransfer: '恢复传输',
  cancelTransfer: '取消传输',
  bandwidthLimit: '带宽限制',
  transferQueue: '传输队列',
};

// 英文翻译
const enUS: Translations = {
  appTitle: 'LAN Transfer Tool',
  appSubtitle: 'Fast and secure file transfer',
  
  deviceInfo: 'Device Info',
  deviceName: 'Device Name',
  deviceType: 'Device Type',
  ipAddress: 'IP Address',
  
  selectFiles: 'Select Files',
  selectedFiles: 'Selected Files',
  noFilesSelected: 'No files selected',
  dragFilesHere: 'Drag files here',
  dragFilesHint: 'Drop files to add them',
  orClickButton: 'Or click button below to select files',
  clearFiles: 'Clear Files',
  removeFile: 'Remove',
  
  scanDevices: 'Scan Devices',
  scanning: 'Scanning...',
  stopScan: 'Stop Scan',
  devicesFound: 'Devices Found',
  noDevicesFound: 'No devices found',
  refreshDevices: 'Refresh Device List',
  
  sendFiles: 'Send Files',
  sendTo: 'Send to',
  transferProgress: 'Transfer Progress',
  transferComplete: 'Transfer Complete',
  transferFailed: 'Transfer Failed',
  transferCancelled: 'Transfer Cancelled',
  
  online: 'Online',
  offline: 'Offline',
  connecting: 'Connecting',
  transferring: 'Transferring',
  paused: 'Paused',
  
  settings: 'Settings',
  darkMode: 'Dark Mode',
  lightMode: 'Light Mode',
  language: 'Language',
  autoStart: 'Auto Start',
  encryption: 'Encryption',
  notifications: 'Notifications',
  
  showWindow: 'Show Window',
  hideWindow: 'Hide Window',
  quit: 'Quit',
  transferHistory: 'Transfer History',
  
  minimizedToTray: 'App minimized to system tray',
  fileReceived: 'File received successfully',
  transferStarted: 'File transfer started',
  
  errorSelectFiles: 'Please select files to send first',
  errorSendFiles: 'Failed to send files',
  errorConnection: 'Connection failed',
  errorFileAccess: 'File access error',
  
  secureTransfer: 'Secure Transfer',
  enterPassword: 'Enter password',
  generateCode: 'Generate Code',
  securityCode: 'Security Code',
  
  folderTransfer: 'Folder Transfer',
  pauseTransfer: 'Pause Transfer',
  resumeTransfer: 'Resume Transfer',
  cancelTransfer: 'Cancel Transfer',
  bandwidthLimit: 'Bandwidth Limit',
  transferQueue: 'Transfer Queue',
};

// 日文翻译
const jaJP: Translations = {
  appTitle: 'LAN転送ツール',
  appSubtitle: '高速で安全なファイル転送',
  
  deviceInfo: 'デバイス情報',
  deviceName: 'デバイス名',
  deviceType: 'デバイスタイプ',
  ipAddress: 'IPアドレス',
  
  selectFiles: 'ファイル選択',
  selectedFiles: '選択済みファイル',
  noFilesSelected: 'ファイルが選択されていません',
  dragFilesHere: 'ファイルをここにドラッグ',
  dragFilesHint: 'ファイルを離してください',
  orClickButton: 'または下のボタンでファイルを選択',
  clearFiles: 'ファイルをクリア',
  removeFile: '削除',
  
  scanDevices: 'デバイススキャン',
  scanning: 'スキャン中...',
  stopScan: 'スキャン停止',
  devicesFound: '発見されたデバイス',
  noDevicesFound: 'デバイスが見つかりません',
  refreshDevices: 'デバイスリスト更新',
  
  sendFiles: 'ファイル送信',
  sendTo: '送信先',
  transferProgress: '転送進行状況',
  transferComplete: '転送完了',
  transferFailed: '転送失敗',
  transferCancelled: '転送キャンセル',
  
  online: 'オンライン',
  offline: 'オフライン',
  connecting: '接続中',
  transferring: '転送中',
  paused: '一時停止',
  
  settings: '設定',
  darkMode: 'ダークモード',
  lightMode: 'ライトモード',
  language: '言語',
  autoStart: '自動起動',
  encryption: '暗号化',
  notifications: '通知',
  
  showWindow: 'ウィンドウを表示',
  hideWindow: 'ウィンドウを非表示',
  quit: '終了',
  transferHistory: '転送履歴',
  
  minimizedToTray: 'アプリがシステムトレイに最小化されました',
  fileReceived: 'ファイル受信完了',
  transferStarted: 'ファイル転送開始',
  
  errorSelectFiles: '送信するファイルを選択してください',
  errorSendFiles: 'ファイル送信に失敗しました',
  errorConnection: '接続に失敗しました',
  errorFileAccess: 'ファイルアクセスエラー',
  
  secureTransfer: 'セキュア転送',
  enterPassword: 'パスワードを入力',
  generateCode: 'コード生成',
  securityCode: 'セキュリティコード',
  
  folderTransfer: 'フォルダ転送',
  pauseTransfer: '転送一時停止',
  resumeTransfer: '転送再開',
  cancelTransfer: '転送キャンセル',
  bandwidthLimit: '帯域制限',
  transferQueue: '転送キュー',
};

// 韩文翻译
const koKR: Translations = {
  appTitle: 'LAN 전송 도구',
  appSubtitle: '빠르고 안전한 파일 전송',
  
  deviceInfo: '장치 정보',
  deviceName: '장치 이름',
  deviceType: '장치 유형',
  ipAddress: 'IP 주소',
  
  selectFiles: '파일 선택',
  selectedFiles: '선택된 파일',
  noFilesSelected: '선택된 파일 없음',
  dragFilesHere: '파일을 여기로 드래그',
  dragFilesHint: '파일을 놓으세요',
  orClickButton: '또는 아래 버튼으로 파일 선택',
  clearFiles: '파일 지우기',
  removeFile: '제거',
  
  scanDevices: '장치 스캔',
  scanning: '스캔 중...',
  stopScan: '스캔 중지',
  devicesFound: '발견된 장치',
  noDevicesFound: '장치를 찾을 수 없음',
  refreshDevices: '장치 목록 새로고침',
  
  sendFiles: '파일 전송',
  sendTo: '전송 대상',
  transferProgress: '전송 진행률',
  transferComplete: '전송 완료',
  transferFailed: '전송 실패',
  transferCancelled: '전송 취소됨',
  
  online: '온라인',
  offline: '오프라인',
  connecting: '연결 중',
  transferring: '전송 중',
  paused: '일시정지',
  
  settings: '설정',
  darkMode: '다크 모드',
  lightMode: '라이트 모드',
  language: '언어',
  autoStart: '자동 시작',
  encryption: '암호화',
  notifications: '알림',
  
  showWindow: '창 표시',
  hideWindow: '창 숨기기',
  quit: '종료',
  transferHistory: '전송 기록',
  
  minimizedToTray: '앱이 시스템 트레이로 최소화됨',
  fileReceived: '파일 수신 완료',
  transferStarted: '파일 전송 시작',
  
  errorSelectFiles: '전송할 파일을 먼저 선택하세요',
  errorSendFiles: '파일 전송 실패',
  errorConnection: '연결 실패',
  errorFileAccess: '파일 접근 오류',
  
  secureTransfer: '보안 전송',
  enterPassword: '비밀번호 입력',
  generateCode: '코드 생성',
  securityCode: '보안 코드',
  
  folderTransfer: '폴더 전송',
  pauseTransfer: '전송 일시정지',
  resumeTransfer: '전송 재개',
  cancelTransfer: '전송 취소',
  bandwidthLimit: '대역폭 제한',
  transferQueue: '전송 대기열',
};

// 翻译字典
const translations: Record<Language, Translations> = {
  'zh-CN': zhCN,
  'en-US': enUS,
  'ja-JP': jaJP,
  'ko-KR': koKR,
};

// 语言检测
export function detectLanguage(): Language {
  const browserLang = navigator.language;
  
  if (browserLang.startsWith('zh')) return 'zh-CN';
  if (browserLang.startsWith('en')) return 'en-US';
  if (browserLang.startsWith('ja')) return 'ja-JP';
  if (browserLang.startsWith('ko')) return 'ko-KR';
  return 'zh-CN'; // 默认中文
}

// 获取翻译文本
export function getTranslations(lang: Language): Translations {
  return translations[lang] || translations['en-US'];
}

// I18n管理类
export class I18nManager {
  private currentLanguage: Language;
  private listeners: ((lang: Language) => void)[] = [];

  constructor() {
    const saved = localStorage.getItem('lan-transfer-language');
    this.currentLanguage = (saved as Language) || detectLanguage();
  }

  getCurrentLanguage(): Language {
    return this.currentLanguage;
  }

  getTranslations(): Translations {
    return getTranslations(this.currentLanguage);
  }

  changeLanguage(lang: Language): void {
    this.currentLanguage = lang;
    localStorage.setItem('lan-transfer-language', lang);
    this.listeners.forEach(listener => listener(lang));
  }

  subscribe(listener: (lang: Language) => void): () => void {
    this.listeners.push(listener);
    return () => {
      const index = this.listeners.indexOf(listener);
      if (index > -1) {
        this.listeners.splice(index, 1);
      }
    };
  }
}

// 创建全局实例
export const i18nManager = new I18nManager(); 