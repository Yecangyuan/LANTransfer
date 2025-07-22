import React, { useState, useEffect } from 'react';
import { Activity, Cpu, HardDrive, Wifi, Monitor } from 'lucide-react';

interface PerformanceData {
  cpu: number;
  memory: number;
  network: {
    upload: number;
    download: number;
  };
  disk: number;
  uptime: string;
}

interface PerformanceMonitorProps {
  darkMode: boolean;
  isVisible: boolean;
  onClose: () => void;
}

export const PerformanceMonitor: React.FC<PerformanceMonitorProps> = ({
  darkMode,
  isVisible,
  onClose
}) => {
  const [performanceData, setPerformanceData] = useState<PerformanceData>({
    cpu: 0,
    memory: 0,
    network: { upload: 0, download: 0 },
    disk: 0,
    uptime: '00:00:00'
  });

  const [history, setHistory] = useState<{
    cpu: number[];
    memory: number[];
    network: { upload: number; download: number }[];
  }>({
    cpu: [],
    memory: [],
    network: []
  });

  useEffect(() => {
    if (!isVisible) return;

    const interval = setInterval(() => {
      // 模拟性能数据
      const newData: PerformanceData = {
        cpu: Math.random() * 100,
        memory: Math.random() * 100,
        network: {
          upload: Math.random() * 1024, // KB/s
          download: Math.random() * 1024
        },
        disk: Math.random() * 100,
        uptime: new Date().toLocaleTimeString()
      };

      setPerformanceData(newData);

      // 更新历史数据
      setHistory(prev => ({
        cpu: [...prev.cpu.slice(-19), newData.cpu],
        memory: [...prev.memory.slice(-19), newData.memory],
        network: [...prev.network.slice(-19), newData.network]
      }));
    }, 1000);

    return () => clearInterval(interval);
  }, [isVisible]);

  const formatBytes = (bytes: number): string => {
    if (bytes < 1024) return `${bytes.toFixed(1)} B/s`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB/s`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB/s`;
  };

  const getStatusColor = (value: number): string => {
    if (value < 30) return darkMode ? 'text-green-400' : 'text-green-600';
    if (value < 70) return darkMode ? 'text-yellow-400' : 'text-yellow-600';
    return darkMode ? 'text-red-400' : 'text-red-600';
  };

  const renderMiniChart = (data: number[], color: string) => {
    const max = Math.max(...data, 1);
    const points = data.map((value, index) => {
      const x = (index / (data.length - 1)) * 100;
      const y = 100 - (value / max) * 100;
      return `${x},${y}`;
    }).join(' ');

    return (
      <svg className="w-16 h-8" viewBox="0 0 100 100" preserveAspectRatio="none">
        <polyline
          fill="none"
          stroke={color}
          strokeWidth="2"
          points={points}
        />
      </svg>
    );
  };

  if (!isVisible) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className={`rounded-xl shadow-2xl p-6 w-96 max-w-full max-h-full overflow-auto ${
        darkMode ? 'bg-gray-800 text-white' : 'bg-white text-gray-800'
      }`}>
        {/* 头部 */}
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center space-x-2">
            <Monitor className="w-5 h-5 text-blue-500" />
            <h2 className="text-lg font-semibold">性能监控</h2>
          </div>
          <button
            onClick={onClose}
            className={`p-1 rounded-lg transition-colors ${
              darkMode 
                ? 'hover:bg-gray-700 text-gray-400' 
                : 'hover:bg-gray-100 text-gray-600'
            }`}
          >
            ✕
          </button>
        </div>

        {/* 性能指标 */}
        <div className="space-y-4">
          {/* CPU */}
          <div className={`p-4 rounded-lg ${
            darkMode ? 'bg-gray-700/50' : 'bg-gray-50'
          }`}>
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <Cpu className="w-4 h-4 text-blue-500" />
                <span className="text-sm font-medium">CPU 使用率</span>
              </div>
              <span className={`text-sm font-mono ${getStatusColor(performanceData.cpu)}`}>
                {performanceData.cpu.toFixed(1)}%
              </span>
            </div>
            <div className={`w-full h-2 rounded-full ${
              darkMode ? 'bg-gray-600' : 'bg-gray-200'
            }`}>
              <div
                className="h-full rounded-full bg-blue-500 transition-all duration-300"
                style={{ width: `${performanceData.cpu}%` }}
              />
            </div>
            <div className="mt-2 flex justify-end">
              {renderMiniChart(history.cpu, '#3b82f6')}
            </div>
          </div>

          {/* 内存 */}
          <div className={`p-4 rounded-lg ${
            darkMode ? 'bg-gray-700/50' : 'bg-gray-50'
          }`}>
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <HardDrive className="w-4 h-4 text-green-500" />
                <span className="text-sm font-medium">内存使用率</span>
              </div>
              <span className={`text-sm font-mono ${getStatusColor(performanceData.memory)}`}>
                {performanceData.memory.toFixed(1)}%
              </span>
            </div>
            <div className={`w-full h-2 rounded-full ${
              darkMode ? 'bg-gray-600' : 'bg-gray-200'
            }`}>
              <div
                className="h-full rounded-full bg-green-500 transition-all duration-300"
                style={{ width: `${performanceData.memory}%` }}
              />
            </div>
            <div className="mt-2 flex justify-end">
              {renderMiniChart(history.memory, '#10b981')}
            </div>
          </div>

          {/* 网络 */}
          <div className={`p-4 rounded-lg ${
            darkMode ? 'bg-gray-700/50' : 'bg-gray-50'
          }`}>
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <Wifi className="w-4 h-4 text-purple-500" />
                <span className="text-sm font-medium">网络活动</span>
              </div>
            </div>
            <div className="space-y-2">
              <div className="flex justify-between items-center">
                <span className="text-xs text-gray-500">上传</span>
                <span className="text-xs font-mono text-purple-500">
                  {formatBytes(performanceData.network.upload)}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-xs text-gray-500">下载</span>
                <span className="text-xs font-mono text-purple-500">
                  {formatBytes(performanceData.network.download)}
                </span>
              </div>
            </div>
          </div>

          {/* 系统信息 */}
          <div className={`p-4 rounded-lg ${
            darkMode ? 'bg-gray-700/50' : 'bg-gray-50'
          }`}>
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center space-x-2">
                <Activity className="w-4 h-4 text-orange-500" />
                <span className="text-sm font-medium">系统信息</span>
              </div>
            </div>
            <div className="space-y-1 text-xs">
              <div className="flex justify-between">
                <span className="text-gray-500">运行时间</span>
                <span className="font-mono">{performanceData.uptime}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">磁盘使用</span>
                <span className="font-mono">{performanceData.disk.toFixed(1)}%</span>
              </div>
            </div>
          </div>
        </div>

        {/* 操作按钮 */}
        <div className="mt-6 flex justify-end space-x-2">
          <button
            onClick={() => {
              setHistory({ cpu: [], memory: [], network: [] });
            }}
            className={`px-3 py-1 text-sm rounded-lg transition-colors ${
              darkMode 
                ? 'bg-gray-700 hover:bg-gray-600 text-gray-300' 
                : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
            }`}
          >
            清空历史
          </button>
          <button
            onClick={onClose}
            className="px-3 py-1 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}; 