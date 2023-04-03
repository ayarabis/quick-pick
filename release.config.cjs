module.exports = {
	branches: ['main'],
	plugins: [
		'@semantic-release/commit-analyzer',
		'@semantic-release/release-notes-generator',
		'@semantic-release/npm',
		'@semantic-release/git',
		[
			'@semantic-release/github',
			{
				assets: [
					{
						path: 'src-tauri/target/release/bundle/dmg/*.dmg',
						label: 'MacOs distribution'
					}
				]
			}
		]
	]
};
