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
