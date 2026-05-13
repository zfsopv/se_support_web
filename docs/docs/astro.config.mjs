// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightThemeNova from 'starlight-theme-nova';
import { ion } from "starlight-ion-theme";
import starlightThemeFlexoki from 'starlight-theme-flexoki'

// https://astro.build/config
export default defineConfig({
	server: {
		host: '0.0.0.0',
		port: 4321,
	},
	integrations: [
		starlight({
			title: '技术支持助手',
			head: [
				{
					tag: 'script',
					attrs: { type: 'module' },
					content: `
						const storageKey = 'starlight-sidebar-collapsed';
						const root = document.documentElement;

						try {
							if (localStorage.getItem(storageKey) === 'true') {
								root.dataset.sidebarCollapsed = 'true';
							}
						} catch {}

						const syncButtonState = (button) => {
							const collapsed = root.dataset.sidebarCollapsed === 'true';
							button.textContent = collapsed ? '>>>' : '<<<';
							button.setAttribute('aria-expanded', String(!collapsed));
							button.setAttribute('aria-label', collapsed ? '展开左侧栏' : '收起左侧栏');
							button.title = collapsed ? '展开左侧栏' : '收起左侧栏';
						};

						const mountSidebarToggle = () => {
							if (!root.hasAttribute('data-has-sidebar')) return;
							const sidebarPane = document.querySelector('.sidebar-pane');
							if (!(sidebarPane instanceof HTMLElement)) return;

							let button = document.querySelector('[data-sidebar-collapse-toggle]');
							if (!(button instanceof HTMLButtonElement)) {
								button = document.createElement('button');
								button.type = 'button';
								button.className = 'sidebar-collapse-toggle';
								button.dataset.sidebarCollapseToggle = '';
								sidebarPane.appendChild(button);
								button.addEventListener('click', () => {
									const collapsed = root.dataset.sidebarCollapsed === 'true';
									if (collapsed) {
										delete root.dataset.sidebarCollapsed;
									} else {
										root.dataset.sidebarCollapsed = 'true';
									}

									try {
										localStorage.setItem(storageKey, String(!collapsed));
									} catch {}

									syncButtonState(button);
								});
							} else if (button.parentElement !== sidebarPane) {
								sidebarPane.appendChild(button);
							}

							syncButtonState(button);
						};

						if (document.readyState === 'loading') {
							document.addEventListener('DOMContentLoaded', mountSidebarToggle, { once: true });
						} else {
							mountSidebarToggle();
						}
					`,
				},
			],
			locales: {
				root: {
					label: '简体中文',
					lang: 'zh-CN',
				},
			},
			customCss: [
				'./src/styles/custom.css',
			],
			social: [{ icon: 'external', label: '官网', href: 'https://www.sophgo.com/' }],
			// plugins: [
			// 	starlightThemeNova(/* options */),
			// ],
			sidebar: [
				{
					label: '技术支持助手',
					link: '/',
				},
				{
					label: '常见问题',
					autogenerate: { directory: 'all' },
				},
				{
					label: 'SE7',
					autogenerate: { directory: 'se7' },
				},
				{
					label: 'SE8',
					autogenerate: { directory: 'se8' },
				},
				{
					label: 'SE9',
					autogenerate: { directory: 'se9' },
				},
				{
					label: '鸣谢列表',
					autogenerate: { directory: 'thanks' },
				},
			],
		}),
	],
});
