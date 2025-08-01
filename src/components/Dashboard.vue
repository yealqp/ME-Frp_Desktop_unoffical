<template>
  <div class="dashboard">
    <!-- 欢迎信息 -->
    <div class="welcome-header">
      <h2 class="welcome-text">欢迎回来，{{ userInfoLoading ? '加载中...' : userInfo.username }}</h2>
    </div>

    <div class="dashboard-grid">
      <!-- 左侧用户信息卡片 -->
      <n-card title="用户信息" :bordered="true" class="user-info-card">
        <div class="user-profile">
          <template v-if="userInfoLoading">
            <div class="user-info-grid">
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 60%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 50%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 40%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 70%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 50%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 45%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 55%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 40%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 45%" />
                </div>
              </div>
              <div class="user-info-item">
                <n-skeleton text :repeat="1" style="width: 30%; font-size: 13px" />
                <div class="user-info-value">
                  <n-skeleton text :repeat="1" style="width: 55%" />
                </div>
              </div>
            </div>
          </template>
          <template v-else>
            <div class="user-info-grid">
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">用户名</n-text>
                <div class="user-info-value">{{ userInfo.username }}</div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">用户 ID</n-text>
                <div class="user-info-value">
                  <n-tag type="warning" :bordered="false" size="small">
                    #{{ userInfo.userId }}
                  </n-tag>
                </div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">实名认证</n-text>
                <div class="user-info-value">
                  <n-tag type="success" :bordered="false" size="small">
                    {{ userInfo.isRealname ? '已实名' : '未实名' }}
                  </n-tag>
                </div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">用户组</n-text>
                <div class="user-info-value">
                  <n-tag type="info" :bordered="false" size="small">
                    {{ userInfo.friendlyGroup }}
                  </n-tag>
                </div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">注册时间</n-text>
                <div class="user-info-value">{{ formatRegTime(userInfo.regTime) }}</div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">注册邮箱</n-text>
                <div class="user-info-value">{{ userInfo.email }}</div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">隧道数量</n-text>
                <div class="user-info-value">{{ userInfo.usedProxies }}/{{ userInfo.maxProxies }}</div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">剩余流量</n-text>
                <div class="user-info-value">{{ (getRemainingTraffic() / 1024).toFixed(2) }} GB</div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">入站带宽</n-text>
                <div class="user-info-value">{{ formatBandwidth(userInfo.inBound) }}</div>
              </div>
              <div class="user-info-item">
                <n-text :style="{ fontSize: '13px' }" depth="3">出站带宽</n-text>
                <div class="user-info-value">{{ formatBandwidth(userInfo.outBound) }}</div>
              </div>
            </div>
          </template>
        </div>
      </n-card>

      <!-- 右侧系统公告卡片 -->
      <div class="announcements-container">
        <template v-if="announcementsLoading">
          <n-card title="系统公告" :bordered="true" class="announcement-card announcement-medium">
            <template #header-extra>
              <n-skeleton text style="width: 80px" />
            </template>
            <n-skeleton text :repeat="3" />
            <n-skeleton text style="width: 60%" />
          </n-card>
          <n-card title="系统公告" :bordered="true" class="announcement-card announcement-small">
            <template #header-extra>
              <n-skeleton text style="width: 80px" />
            </template>
            <n-skeleton text :repeat="2" />
            <n-skeleton text style="width: 40%" />
          </n-card>
        </template>
        <template v-else>
          <n-card v-for="announcement in announcements" :key="announcement.id" :title="announcement.title"
            :bordered="true" class="announcement-card" :class="getAnnouncementCardClass(announcement)">
            <template #header-extra>
              <span class="announcement-date">{{ announcement.date }}</span>
            </template>
            <div v-html="parseMarkdownContent(announcement.content)"></div>
          </n-card>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { marked } from 'marked'
import { useMessage } from 'naive-ui'

// 用户信息数据结构
interface UserInfo {
  email: string;
  friendlyGroup: string;
  group: string;
  inBound: number;
  isRealname: boolean;
  maxProxies: number;
  outBound: number;
  regTime: number;
  status: number;
  todaySigned: boolean;
  traffic: number;
  usedProxies: number;
  userId: number;
  username: string;
}

interface Announcement {
  id: number;
  title: string;
  content: string;
  date: string;
}

// 用户信息响应式数据
const userInfo = ref<UserInfo>({
  email: '',
  friendlyGroup: '',
  group: '',
  inBound: 0,
  isRealname: false,
  maxProxies: 0,
  outBound: 0,
  regTime: 0,
  status: 0,
  todaySigned: false,
  traffic: 0,
  usedProxies: 0,
  userId: 0,
  username: '加载中...'
});

// 系统公告数据
const announcements = ref<Announcement[]>([])

// 加载状态
const userInfoLoading = ref(true)
const announcementsLoading = ref(true)
const message = useMessage()

// 缓存相关
const CACHE_DURATION = 5 * 60 * 1000; // 5分钟缓存时间
const userInfoCache = ref<{ data: UserInfo | null; timestamp: number }>({ data: null, timestamp: 0 });
const announcementsCache = ref<{ data: Announcement[]; timestamp: number }>({ data: [], timestamp: 0 });

// 检查缓存是否有效
const isCacheValid = (timestamp: number): boolean => {
  return Date.now() - timestamp < CACHE_DURATION;
};





// 获取系统公告
const fetchAnnouncements = async (forceRefresh: boolean = false) => {
  // 检查缓存是否有效且不强制刷新
  if (!forceRefresh && announcementsCache.value.data.length > 0 && isCacheValid(announcementsCache.value.timestamp)) {
    announcements.value = announcementsCache.value.data;
    announcementsLoading.value = false;
    console.log('使用缓存的系统公告');
    return;
  }

  announcementsLoading.value = true;
  try {
    // 调用后端API命令获取系统公告
    const noticeData: any = await invoke('api_get_announcements');

    console.log('获取系统公告成功');

    // 处理公告数据
    if (noticeData) {
      // 如果返回的是字符串，将其作为单个公告处理
      if (typeof noticeData === 'string') {
        announcements.value = [
          {
            id: 1,
            title: '系统公告',
            content: noticeData,
            date: new Date().toISOString().split('T')[0]
          }
        ];
      } else if (Array.isArray(noticeData) && noticeData.length > 0) {
        // 如果返回的是数组格式
        announcements.value = noticeData.map((item: any, index: number) => ({
          id: index + 1,
          title: item.title || item.subject || '系统公告',
          content: item.content || item.message || item.body || '暂无内容',
          date: item.date || item.created_at || item.time || new Date().toISOString().split('T')[0]
        }));
      } else if ((noticeData as any).data) {
        // 如果返回的是对象格式
        const noticesArray = Array.isArray((noticeData as any).data) ? (noticeData as any).data : [(noticeData as any).data];

        announcements.value = noticesArray.map((notice: any, index: number) => ({
          id: notice.id || index + 1,
          title: notice.title || notice.name || notice.subject || '系统公告',
          content: notice.content || notice.message || notice.text || '',
          date: notice.date || notice.created_at || notice.time || new Date().toISOString().split('T')[0]
        }));
      } else {
        // 如果是其他格式，尝试直接处理
        announcements.value = [
          {
            id: 1,
            title: '系统公告',
            content: String(noticeData),
            date: new Date().toISOString().split('T')[0],
          }
        ];
      }
    } else {
      // 如果没有公告数据，显示提示信息
      announcements.value = [
        {
          id: 1,
          title: '暂无公告',
          content: '当前没有系统公告。',
          date: new Date().toISOString().split('T')[0],
        }
      ];
    }

    // 更新缓存
    announcementsCache.value = {
      data: announcements.value,
      timestamp: Date.now()
    };

  } catch (error) {
    console.error('获取系统公告失败', error);
    // 显示完整的错误信息
    const errorMessage = error && typeof error === 'string' ? error :
      error && typeof error === 'object' && 'message' in error ?
        (error as any).message : '请检查网络连接';
    message.error(`获取系统公告失败: ${errorMessage}`);

    // 如果有缓存数据，使用缓存数据
    if (announcementsCache.value.data.length > 0) {
      announcements.value = announcementsCache.value.data;
      console.log('API请求失败，使用缓存的系统公告');
    } else {
      // 如果没有缓存，显示错误信息
      announcements.value = [
        {
          id: 1,
          title: '获取公告失败',
          content: '无法从服务器获取最新公告，请检查网络连接或稍后重试。',
          date: new Date().toISOString().split('T')[0],
        }
      ];
    }
  } finally {
    announcementsLoading.value = false;
  }
}







// 格式化带宽（单位：Mbps，响应数值/128是显示数值）
const formatBandwidth = (value: number): string => {
  if (value === 0) return '0 Mbps';
  const mbps = value / 128;
  return parseFloat(mbps.toFixed(2)) + ' Mbps';
};

// 获取用户信息
const fetchUserInfo = async (forceRefresh: boolean = false) => {
  // 检查缓存是否有效且不强制刷新
  if (!forceRefresh && userInfoCache.value.data && isCacheValid(userInfoCache.value.timestamp)) {
    userInfo.value = userInfoCache.value.data;
    userInfoLoading.value = false;
    console.log('使用缓存的用户信息');
    return;
  }

  userInfoLoading.value = true;
  try {
    // 调用后端API命令获取用户信息
    const userDetailInfo: any = await invoke('api_get_user_info');
    userInfo.value = userDetailInfo as UserInfo;

    // 更新缓存
    userInfoCache.value = {
      data: userDetailInfo as UserInfo,
      timestamp: Date.now()
    };

    console.log('获取用户信息成功');
  } catch (error) {
    console.error('获取用户信息失败', error);
    // 显示完整的错误信息
    const errorMessage = error && typeof error === 'string' ? error :
      error && typeof error === 'object' && 'message' in error ?
        (error as any).message : '请检查网络连接';
    message.error(`获取用户信息失败: ${errorMessage}`);
    // 如果有缓存数据，使用缓存数据
    if (userInfoCache.value.data) {
      userInfo.value = userInfoCache.value.data;
      console.log('API请求失败，使用缓存的用户信息');
    }
  } finally {
    userInfoLoading.value = false;
  }
};

// 格式化注册时间
const formatRegTime = (timestamp: number): string => {
  const date = new Date(timestamp * 1000);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  });
};

// 获取剩余流量（直接使用API返回的traffic字段）
const getRemainingTraffic = (): number => {
  // API响应中的traffic字段就是剩余流量
  return userInfo.value.traffic || 0;
};

// 解析Markdown内容并处理转义字符
const parseMarkdownContent = (content: string): string => {
  if (!content) return '';

  // 处理转义字符
  const unescapedContent = content
    .replace(/\\n/g, '\n')
    .replace(/\\t/g, '\t')
    .replace(/\\r/g, '\r')
    .replace(/\\\\/g, '\\')
    .replace(/\\"/g, '"')
    .replace(/\\'/g, "'")
    .replace(/\\&/g, '&')
    .replace(/\\</g, '<')
    .replace(/\\>/g, '>');

  // 解析Markdown
  try {
    return marked(unescapedContent, {
      breaks: true,
      gfm: true
    }) as string;
  } catch (error) {
    console.error('Markdown解析失败:', error);
    return unescapedContent.replace(/\n/g, '<br>');
  }
};



// 根据内容长度计算公告卡片的CSS类
const getAnnouncementCardClass = (announcement: any): string => {
  const contentLength = announcement.content?.length || 0;

  if (contentLength > 200) {
    return 'announcement-large';
  } else if (contentLength > 100) {
    return 'announcement-medium';
  } else {
    return 'announcement-small';
  }
};

// 组件挂载时获取用户信息和系统公告
onMounted(() => {
  fetchUserInfo();
  fetchAnnouncements();
});
</script>

<style scoped>
.dashboard {
  padding: 0;
}

/* 欢迎信息样式 */
.welcome-header {
  margin-bottom: 20px;
  padding: 0 4px;
}

.welcome-text {
  font-size: 28px;
  font-weight: 600;
  color: #333;
  margin: 0;
  text-align: left;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  align-items: start;
}

@media (max-width: 768px) {
  .dashboard-grid {
    grid-template-columns: 1fr;
  }
}

/* 用户信息卡片样式 */
.user-profile {
  display: flex;
  flex-direction: column;
}

.user-info-grid {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 30px;
  row-gap: 10px;
  column-gap: 60px;
}

.user-info-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 16px;
  background: #18181c;
  border-radius: 8px;
}

.user-info-value {
  color: #ffffff;
  font-weight: 600;
  font-size: 14px;
  margin-top: 4px;
}

@media (max-width: 768px) {
  .user-info-grid {
    grid-template-columns: 1fr;
  }
}

/* 系统公告容器样式 */
.announcements-container {
  padding-right: 8px;
}



/* 系统公告卡片样式 */
.announcement-card {
  margin-bottom: 16px;
  transition: all 0.3s ease;
}

.announcement-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.announcement-card:last-child {
  margin-bottom: 0;
}

/* 动态卡片大小 */
.announcement-small {
  min-height: 120px;
}

.announcement-medium {
  min-height: 160px;
}

.announcement-large {
  min-height: 200px;
}

/* 卡片头部额外内容样式 */
.announcement-card .n-card-header__extra {
  display: flex;
  align-items: center;
  gap: 12px;
}

.announcement-date {
  color: #999;
  font-size: 12px;
  white-space: nowrap;
}

.announcement-card .n-card__content {
  color: #666;
  line-height: 1.6;
  font-size: 14px;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

/* Markdown内容样式 */
.announcement-card .n-card__content h1,
.announcement-card .n-card__content h2,
.announcement-card .n-card__content h3,
.announcement-card .n-card__content h4,
.announcement-card .n-card__content h5,
.announcement-card .n-card__content h6 {
  margin: 8px 0 4px 0;
  color: #333;
}

.announcement-card .n-card__content p {
  margin: 4px 0;
}

.announcement-card .n-card__content code {
  background: #f5f5f5;
  padding: 2px 4px;
  border-radius: 3px;
  font-family: 'Courier New', monospace;
  font-size: 13px;
}

.announcement-card .n-card__content pre {
  background: #f5f5f5;
  padding: 8px;
  border-radius: 4px;
  overflow-x: auto;
  margin: 8px 0;
}

.announcement-card .n-card__content blockquote {
  border-left: 4px solid #ddd;
  margin: 8px 0;
  padding-left: 12px;
  color: #666;
}

.announcement-card .n-card__content ul,
.announcement-card .n-card__content ol {
  margin: 4px 0;
  padding-left: 20px;
}

.announcement-card .n-card__content a {
  color: #1890ff;
  text-decoration: none;
}

.announcement-card .n-card__content a:hover {
  text-decoration: underline;
}
</style>