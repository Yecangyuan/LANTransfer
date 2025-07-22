import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/api/dialog';
import { 
  Wifi, 
  Users, 
  Upload, 
  Download, 
  FolderOpen, 
  Send, 
  Smartphone,
  Monitor,
  RefreshCw,
  Check,
  AlertCircle,
  X
} from 'lucide-react';

interface Device {
  id: string;
  name: string;
  ip: string;
  device_type: string;
  is_online: boolean;
}

interface TransferProgress {
  file_name: string;
  progress: number;
  status: 'sending' | 'receiving' | 'completed' | 'failed';
}

function App() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [isScanning, setIsScanning] = useState(false);
  const [selectedFiles, setSelectedFiles] = useState<string[]>([]);
  const [transferProgress, setTransferProgress] = useState<TransferProgress[]>([]);
  const [myDeviceInfo, setMyDeviceInfo] = useState<Device | null>(null);

  useEffect(() => {
    // 监听设备发现事件
    const unlisten = listen('device-discovered', (event) => {
      const device = event.payload as Device;
      setDevices(prev => {
        const exists = prev.find(d => d.id === device.id);
        if (exists) {
          return prev.map(d => d.id === device.id ? device : d);
        }
        return [...prev, device];
      });
    });

    // 监听传输进度事件
    const unlistenProgress = listen('transfer-progress', (event) => {
      const progress = event.payload as TransferProgress;
      setTransferProgress(prev => {
        const exists = prev.find(p => p.file_name === progress.file_name);
        if (exists) {
          return prev.map(p => p.file_name === progress.file_name ? progress : p);
        }
        return [...prev, progress];
      });
    });

    // 获取本设备信息
    initializeDevice();

    return () => {
      unlisten.then(f => f());
      unlistenProgress.then(f => f());
    };
  }, []);

  const initializeDevice = async () => {
    try {
      const deviceInfo = await invoke<Device>('get_device_info');
      setMyDeviceInfo(deviceInfo);
    } catch (error) {
      console.error('Failed to get device info:', error);
    }
  };

  const startScanning = async () => {
    setIsScanning(true);
    try {
      await invoke('start_device_scan');
      setTimeout(() => setIsScanning(false), 3000); // 停止扫描动画
    } catch (error) {
      console.error('Failed to start scanning:', error);
      setIsScanning(false);
    }
  };

  const selectFiles = async () => {
    try {
      const selected = await open({
        multiple: true,
        filters: [{
          name: '所有文件',
          extensions: ['*']
        }]
      });
      
      if (selected && Array.isArray(selected)) {
        setSelectedFiles(selected);
      } else if (selected && typeof selected === 'string') {
        setSelectedFiles([selected]);
      }
    } catch (error) {
      console.error('Failed to select files:', error);
    }
  };

  const sendFiles = async (targetDeviceId: string) => {
    if (selectedFiles.length === 0) {
      alert('请先选择要发送的文件');
      return;
    }

    try {
      await invoke('send_files', {
        targetDeviceId,
        filePaths: selectedFiles
      });
    } catch (error) {
      console.error('Failed to send files:', error);
      alert('发送文件失败: ' + error);
    }
  };

  const getDeviceIcon = (deviceType: string) => {
    switch (deviceType.toLowerCase()) {
      case 'mobile':
        return <Smartphone className="w-5 h-5" />;
      case 'desktop':
      case 'laptop':
        return <Monitor className="w-5 h-5" />;
      default:
        return <Monitor className="w-5 h-5" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'completed':
        return 'text-green-600';
      case 'failed':
        return 'text-red-600';
      case 'sending':
      case 'receiving':
        return 'text-blue-600';
      default:
        return 'text-gray-600';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <Check className="w-4 h-4" />;
      case 'failed':
        return <X className="w-4 h-4" />;
      case 'sending':
        return <Upload className="w-4 h-4" />;
      case 'receiving':
        return <Download className="w-4 h-4" />;
      default:
        return <AlertCircle className="w-4 h-4" />;
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-6">
      <div className="max-w-4xl mx-auto">
        {/* 头部 */}
        <div className="bg-white rounded-xl shadow-lg p-6 mb-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="p-2 bg-blue-500 text-white rounded-lg">
                <Wifi className="w-6 h-6" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-gray-800">局域网传输工具</h1>
                <p className="text-gray-600">快速安全的文件传输</p>
              </div>
            </div>
            
            {myDeviceInfo && (
              <div className="text-right">
                <p className="text-sm text-gray-600">当前设备</p>
                <p className="font-medium text-gray-800">{myDeviceInfo.name}</p>
                <p className="text-sm text-gray-500">{myDeviceInfo.ip}</p>
              </div>
            )}
          </div>
        </div>

        {/* 文件选择区域 */}
        <div className="bg-white rounded-xl shadow-lg p-6 mb-6">
          <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
            <FolderOpen className="w-5 h-5 mr-2 text-blue-500" />
            选择文件
          </h2>
          
          <div className="flex items-center space-x-4 mb-4">
            <button
              onClick={selectFiles}
              className="flex items-center space-x-2 bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transition-colors"
            >
              <FolderOpen className="w-4 h-4" />
              <span>选择文件</span>
            </button>
            
            <span className="text-gray-600">
              {selectedFiles.length > 0 ? `已选择 ${selectedFiles.length} 个文件` : '未选择文件'}
            </span>
          </div>

          {selectedFiles.length > 0 && (
            <div className="bg-gray-50 rounded-lg p-4 max-h-32 overflow-y-auto">
              {selectedFiles.map((file, index) => (
                <div key={index} className="text-sm text-gray-700 truncate">
                  {file.split('/').pop() || file}
                </div>
              ))}
            </div>
          )}
        </div>

        {/* 设备发现区域 */}
        <div className="bg-white rounded-xl shadow-lg p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold text-gray-800 flex items-center">
              <Users className="w-5 h-5 mr-2 text-green-500" />
              发现的设备
            </h2>
            
            <button
              onClick={startScanning}
              disabled={isScanning}
              className="flex items-center space-x-2 bg-green-500 hover:bg-green-600 disabled:bg-green-300 text-white px-4 py-2 rounded-lg transition-colors"
            >
              <RefreshCw className={`w-4 h-4 ${isScanning ? 'animate-spin' : ''}`} />
              <span>{isScanning ? '扫描中...' : '扫描设备'}</span>
            </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {devices.filter(d => d.id !== myDeviceInfo?.id).map((device) => (
              <div
                key={device.id}
                className={`p-4 rounded-lg border-2 transition-all ${
                  device.is_online 
                    ? 'border-green-200 bg-green-50 hover:border-green-300' 
                    : 'border-gray-200 bg-gray-50'
                }`}
              >
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center space-x-2">
                    {getDeviceIcon(device.device_type)}
                    <span className="font-medium text-gray-800">{device.name}</span>
                  </div>
                  <div className={`w-2 h-2 rounded-full ${device.is_online ? 'bg-green-500' : 'bg-gray-400'}`} />
                </div>
                
                <p className="text-sm text-gray-600 mb-3">{device.ip}</p>
                
                <button
                  onClick={() => sendFiles(device.id)}
                  disabled={!device.is_online || selectedFiles.length === 0}
                  className="w-full flex items-center justify-center space-x-2 bg-blue-500 hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed text-white px-3 py-2 rounded-lg transition-colors text-sm"
                >
                  <Send className="w-4 h-4" />
                  <span>发送文件</span>
                </button>
              </div>
            ))}
            
            {devices.filter(d => d.id !== myDeviceInfo?.id).length === 0 && (
              <div className="col-span-full text-center py-8 text-gray-500">
                <Users className="w-12 h-12 mx-auto mb-3 text-gray-300" />
                <p>未发现设备</p>
                <p className="text-sm">点击扫描设备开始查找</p>
              </div>
            )}
          </div>
        </div>

        {/* 传输进度区域 */}
        {transferProgress.length > 0 && (
          <div className="bg-white rounded-xl shadow-lg p-6">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
              <Download className="w-5 h-5 mr-2 text-purple-500" />
              传输进度
            </h2>
            
            <div className="space-y-3">
              {transferProgress.map((progress, index) => (
                <div key={index} className="border rounded-lg p-4">
                  <div className="flex items-center justify-between mb-2">
                    <span className="font-medium text-gray-800">{progress.file_name}</span>
                    <div className={`flex items-center space-x-1 ${getStatusColor(progress.status)}`}>
                      {getStatusIcon(progress.status)}
                      <span className="text-sm capitalize">{progress.status}</span>
                    </div>
                  </div>
                  
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${progress.progress}%` }}
                    />
                  </div>
                  
                  <div className="text-right text-sm text-gray-600 mt-1">
                    {progress.progress.toFixed(1)}%
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App; 