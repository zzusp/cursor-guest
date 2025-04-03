import './style.css';

// 图标定义
const ICONS = {
  refresh: `<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
  </svg>`,
  close: `<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
  </svg>`,
  check: `<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
  </svg>`,
  info: `<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
  </svg>`,
  warning: `<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
  </svg>`,
};

// 操作步骤定义
const STEPS = [
  { id: 'check-process', name: '检查Cursor进程', english: 'Check Cursor Process' },
  { id: 'backup-config', name: '备份配置文件', english: 'Backup Config File' },
  { id: 'generate-ids', name: '生成新的ID', english: 'Generate New IDs' },
  { id: 'update-registry', name: '更新系统注册表', english: 'Update System Registry' },
  { id: 'update-config', name: '更新配置文件', english: 'Update Config File' }
];

// 步骤状态
const STEP_STATUS = {
  PENDING: 'pending',
  IN_PROGRESS: 'in-progress',
  COMPLETED: 'completed',
  ERROR: 'error'
};

// 步骤状态显示文本
const STATUS_TEXT = {
  [STEP_STATUS.PENDING]: '待处理',
  [STEP_STATUS.IN_PROGRESS]: '处理中',
  [STEP_STATUS.COMPLETED]: '已完成',
  [STEP_STATUS.ERROR]: '出错'
};

// 步骤状态显示文本（英文）
const STATUS_TEXT_EN = {
  [STEP_STATUS.PENDING]: 'Pending',
  [STEP_STATUS.IN_PROGRESS]: 'In Progress',
  [STEP_STATUS.COMPLETED]: 'Completed',
  [STEP_STATUS.ERROR]: 'Error'
};

// 使用异步IIFE初始化应用
(async function() {
  try {
    // 导入Tauri API
    const { invoke } = await import('@tauri-apps/api/tauri');
    const { listen } = await import('@tauri-apps/api/event');
    const { convertFileSrc } = await import('@tauri-apps/api/tauri');
    
    // 监听步骤状态更新事件
    await listen('step-status-update', (event) => {
      console.log('步骤状态更新:', event.payload);
      const { stepId, status } = event.payload;
      updateStepStatus(stepId, status);
    });
    
    // 动态导入Tauri API
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/tauri');
    
    // 渲染UI
    const app = document.querySelector('#app');
    app.innerHTML = `
      <div class="container">
        <div class="card">
          <div class="app-title">
            <img src="/icon.png" class="app-icon" alt="Cursor Guest Icon" />
            <h1>Cursor Guest</h1>
          </div>
          <div id="status-message" class="status">Ready to modify Cursor IDs</div>
          <div class="controls">
            <button id="modify-btn" class="primary-btn">
              <span class="button-icon">${ICONS.refresh}</span>
              Modify IDs
            </button>
            <button id="close-cursor-btn" class="secondary-btn">
              <span class="button-icon">${ICONS.close}</span>
              Close Cursor
            </button>
          </div>
          <div id="progress" class="progress hidden">
            <div class="progress-bar"></div>
          </div>
          
          <div id="steps-container" class="steps-container">
            <ul id="steps-list" class="steps-list">
              ${STEPS.map(step => `
                <li id="${step.id}" class="step-item step-pending">
                  <span class="step-name" title="${step.english}">${step.name}</span>
                  <span class="step-status" title="${STATUS_TEXT_EN[STEP_STATUS.PENDING]}">${STATUS_TEXT[STEP_STATUS.PENDING]}</span>
                </li>
              `).join('')}
            </ul>
          </div>
          
          <div id="config-info" class="config-info hidden">
            <h3>Configuration Details:</h3>
            <pre id="config-content"></pre>
          </div>
          <div class="version">v0.0.1</div>
        </div>
      </div>
    `;

    // 设置事件处理器
    const modifyBtn = document.getElementById('modify-btn');
    const closeCursorBtn = document.getElementById('close-cursor-btn');
    const statusMessage = document.getElementById('status-message');
    const progressBar = document.getElementById('progress');
    const stepsContainer = document.getElementById('steps-container');
    const configInfo = document.getElementById('config-info');
    const configContent = document.getElementById('config-content');

    // 更新步骤状态函数
    function updateStepStatus(stepId, status) {
      const stepElement = document.getElementById(stepId);
      if (!stepElement) return;
      
      // 移除所有状态类
      stepElement.classList.remove('step-pending', 'step-in-progress', 'step-completed', 'step-error');
      
      // 添加新状态类
      stepElement.classList.add(`step-${status}`);
      
      // 更新状态文本
      const statusElement = stepElement.querySelector('.step-status');
      statusElement.textContent = STATUS_TEXT[status];
      statusElement.title = STATUS_TEXT_EN[status]; // 添加英文状态作为title提示
    }

    // 使所有步骤为待处理状态
    function resetSteps() {
      STEPS.forEach(step => {
        updateStepStatus(step.id, STEP_STATUS.PENDING);
      });
    }

    // 修改ID按钮点击事件
    modifyBtn.addEventListener('click', async () => {
      try {
        // 重置步骤状态
        resetSteps();
        
        // 禁用按钮
        modifyBtn.disabled = true;
        closeCursorBtn.disabled = true;
        
        // 更新状态
        statusMessage.textContent = 'Modifying Cursor IDs...';
        statusMessage.className = 'status';
        progressBar.classList.remove('hidden');
        
        // 显示步骤容器
        stepsContainer.classList.remove('hidden');
        
        // 隐藏配置信息
        configInfo.classList.add('hidden');
        
        // 执行步骤序列
        const stepsToExecute = ['check-process', 'backup-config', 'generate-ids', 'update-registry', 'update-config'];
        
        try {
          // 执行步骤1: 检查Cursor进程
          updateStepStatus('check-process', STEP_STATUS.IN_PROGRESS);
          await invoke('close_cursor_processes');
          updateStepStatus('check-process', STEP_STATUS.COMPLETED);
          
          // 执行其余步骤
          for (let i = 1; i < stepsToExecute.length; i++) {
            const stepId = stepsToExecute[i];
            
            // 已通过事件处理的步骤无需手动更新状态
            if (stepId !== 'update-registry') {
              updateStepStatus(stepId, STEP_STATUS.PENDING);
            }
            
            // 如果是最后一步，则调用实际的后端函数
            if (stepId === stepsToExecute[stepsToExecute.length - 1]) {
              const result = await invoke('modify_cursor_ids');
              // 保存结果用于后续显示
              window.modifyResult = result;
            }
            
            // 如果步骤是update-registry且操作失败，显示管理员权限提示
            if (stepId === 'update-registry' && 
                document.getElementById('update-registry').classList.contains('step-error')) {
              statusMessage.innerHTML = `
                <span class="button-icon">${ICONS.warning}</span>
                Registry update requires admin rights. Please run as administrator.
              `;
              statusMessage.classList.add('warning');
              break;
            }
          }
          
          // 成功后的UI更新
          statusMessage.innerHTML = `
            <span class="button-icon">${ICONS.check}</span>
            Cursor IDs modified successfully!
          `;
          statusMessage.classList.add('success');
          
          // 显示配置信息
          setTimeout(() => {
            configInfo.classList.remove('hidden');
            displayConfigDetails(window.modifyResult, configContent);
          }, 500);
          
        } catch (error) {
          // 如果任一步骤失败，将当前步骤标记为错误
          const currentStepIndex = stepsToExecute.findIndex(stepId => 
            document.getElementById(stepId).classList.contains('step-in-progress'));
          
          if (currentStepIndex !== -1) {
            updateStepStatus(stepsToExecute[currentStepIndex], STEP_STATUS.ERROR);
          }
          
          throw error; // 重新抛出错误以便被外层catch块捕获
        }
        
      } catch (error) {
        console.error('Error modifying IDs:', error);
        statusMessage.textContent = `Error: ${error}`;
        statusMessage.classList.add('error');
      } finally {
        // 恢复按钮状态并隐藏进度条
        progressBar.classList.add('hidden');
        setTimeout(() => {
          modifyBtn.disabled = false;
          closeCursorBtn.disabled = false;
        }, 800);
      }
    });

    // 关闭Cursor按钮点击事件
    closeCursorBtn.addEventListener('click', async () => {
      try {
        // 禁用按钮
        modifyBtn.disabled = true;
        closeCursorBtn.disabled = true;
        
        // 更新状态
        statusMessage.textContent = 'Closing Cursor...';
        statusMessage.className = 'status';
        progressBar.classList.remove('hidden');
        
        // 隐藏配置信息
        configInfo.classList.add('hidden');
        
        // 调用Tauri后端
        await invoke('close_cursor_processes');
        
        // 获取进程状态
        const processStatus = await invoke('check_cursor_process_status');
        
        // 根据进程状态显示对应消息
        if (processStatus.includes("未发现运行中的Cursor进程")) {
          statusMessage.innerHTML = `
            <span class="button-icon">${ICONS.info}</span>
            No running Cursor process found.
          `;
        } else if (processStatus.includes("成功关闭Cursor进程")) {
          statusMessage.innerHTML = `
            <span class="button-icon">${ICONS.check}</span>
            Cursor has been closed successfully.
          `;
        } else {
          // 如果是其他状态，直接显示原始消息
          statusMessage.innerHTML = `
            <span class="button-icon">${ICONS.info}</span>
            ${processStatus}
          `;
        }
        statusMessage.classList.add('success');
      } catch (error) {
        console.error('Error closing Cursor:', error);
        statusMessage.textContent = `Error: ${error}`;
        statusMessage.classList.add('error');
      } finally {
        // 恢复按钮状态
        progressBar.classList.add('hidden');
        setTimeout(() => {
          modifyBtn.disabled = false;
          closeCursorBtn.disabled = false;
        }, 800);
      }
    });
    
    // 格式化并显示配置详情
    function displayConfigDetails(config, element) {
      if (!config) return;
      
      const formattedConfig = {
        'Machine ID': config.telemetry_machine_id,
        'Mac Machine ID': config.telemetry_mac_machine_id,
        'Device ID': config.telemetry_dev_device_id,
        'SQM ID': config.telemetry_sqm_id
      };
      
      element.textContent = JSON.stringify(formattedConfig, null, 2);
    }
    
  } catch (error) {
    console.error('Failed to initialize Tauri API:', error);
    
    // 如果Tauri API初始化失败，显示错误信息
    app.innerHTML = `
      <div class="container">
        <div class="card">
          <h1>Cursor Guest</h1>
          <div class="status error">Failed to initialize Tauri API</div>
          <div class="status">Please make sure you're running with 'npm run tauri dev'</div>
          <pre class="error-details">${error.toString()}</pre>
        </div>
      </div>
    `;
  }
})(); 