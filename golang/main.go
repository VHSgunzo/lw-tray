package main

import (
	"embed"
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"strings"
	"syscall"
	"time"

	"github.com/getlantern/systray"

	"github.com/leonelquinteros/gotext"
)

const TDOM = "lw_tray"

//go:embed locales/*
var locales embed.FS

var lw_source = os.Getenv("LW_SOURCE")
var lw_apps_dir = os.Getenv("LW_APPS_DIR")

type lw_app struct {
	item *systray.MenuItem
	cfg  *systray.MenuItem
	done chan struct{}
}

func shellspawn(command string) {
	cmd := exec.Command("sh", "-c", command)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	go cmd.Run()
}

func lwexec(arg string) {
	shellspawn(lw_source + " " + arg)
}

func detect_lang() string {
	lang_envs := []string{"LC_ALL", "LANG"}
	for _, lang_env := range lang_envs {
		value := os.Getenv(lang_env)
		if value != "" {
			parts := strings.Split(value, ".")
			return parts[0]
		}
	}
	return "en_US"
}

func load_translation(lang string) error {
	trans_dir, err := os.MkdirTemp("", "trans")
	if err != nil {
		return fmt.Errorf("failed to create translations dir: %w", err)
	}
	defer os.RemoveAll(trans_dir)
	lang_dir := filepath.Join(trans_dir, lang)
	os.Mkdir(lang_dir, 0755)
	embed_mo := fmt.Sprintf("locales/%s/LC_MESSAGES/%s.mo", lang, TDOM)
	data, err := locales.ReadFile(embed_mo)
	if err != nil {
		return fmt.Errorf("failed to read embedded locale file %s: %v", embed_mo, err)
	}
	temp_mo := filepath.Join(lang_dir, TDOM+".mo")
	err = os.WriteFile(temp_mo, data, 0644)
	if err != nil {
		return fmt.Errorf("failed to write temp .mo file: %v", err)
	}
	gotext.Configure(trans_dir, lang, TDOM)
	return nil
}

func is_file_exists(filename string) bool {
	info, err := os.Stat(filename)
	if os.IsNotExist(err) {
		return false
	}
	return !info.IsDir()
}

func get_icon_bytes(icon string) []byte {
	var icon_bytes []byte
	icon_bytes = append(icon_bytes, 0)
	if is_file_exists(icon) {
		icon_bytes, _ = os.ReadFile(icon)
	}
	return icon_bytes
}

func ls_dir(path string) ([]os.DirEntry, error) {
	files, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}
	return files, nil
}

func get_lw_apps() []string {
	entries, _ := ls_dir(lw_apps_dir)
	var apps []string
	for _, entry := range entries {
		if strings.HasSuffix(entry.Name(), ".desktop") {
			apps = append(apps, strings.TrimSuffix(entry.Name(), ".desktop"))
		}
	}
	return apps
}

func ls_lw_apps() string {
	entries, _ := ls_dir(lw_apps_dir)
	var apps string
	for _, entry := range entries {
		if strings.HasSuffix(entry.Name(), ".desktop") {
			apps += entry.Name()
		}
	}
	return apps
}

func on_exit() {}

func tray() {
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, syscall.SIGINT, syscall.SIGTERM)

	icon := os.Getenv("LW_EXE_ICON")
	if icon == "" {
		icon = os.Getenv("LW_DEF_ICO_PTH")
	}
	systray.SetIcon(get_icon_bytes(icon))

	title := os.Getenv("EXE_NAME")
	if title == "" {
		title = "Lux Wine"
	}
	systray.SetTitle(title)

	apps_items := make(map[string]lw_app)
	lw_apps := get_lw_apps()
	apps := systray.AddMenuItem(gotext.Get("Apps"), "")

	open := systray.AddMenuItem(gotext.Get("Open"), "")
	open_luxwine := open.AddSubMenuItem(gotext.Get("Lux Wine"), "")
	open_debug := open.AddSubMenuItem(gotext.Get("DEBUG"), "")
	if os.Getenv("LU_EXE") != "" {
		open.Hide()
	}

	wine := systray.AddMenuItem(gotext.Get("Wine"), "")
	wine_explorer := wine.AddSubMenuItem(gotext.Get("Explorer"), "")
	wine_taskmgr := wine.AddSubMenuItem(gotext.Get("Task manager"), "")
	wine_cmd := wine.AddSubMenuItem(gotext.Get("CMD"), "")
	wine_control := wine.AddSubMenuItem(gotext.Get("Control panel"), "")
	wine_cfg := wine.AddSubMenuItem(gotext.Get("Settings"), "")
	wine_regedit := wine.AddSubMenuItem(gotext.Get("Registry editor"), "")
	wine_uninstaller := wine.AddSubMenuItem(gotext.Get("Uninstaller"), "")

	kill := systray.AddMenuItem(gotext.Get("Kill"), "")
	kill_exe := kill.AddSubMenuItem(gotext.Get("EXE"), "")
	kill_wine := kill.AddSubMenuItem(gotext.Get("Wine"), "")
	kill_shell := kill.AddSubMenuItem(gotext.Get("SHELL"), "")
	kill_all := kill.AddSubMenuItem(gotext.Get("ALL"), "")

	pfx := systray.AddMenuItem(gotext.Get("Prefix"), "")
	pfx_open := pfx.AddSubMenuItem(gotext.Get("Open drive C:\\"), "")
	pfx_backup := pfx.AddSubMenuItem(gotext.Get("Backup"), "")
	pfx_restore := pfx.AddSubMenuItem(gotext.Get("Restore"), "")
	pfx_clear := pfx.AddSubMenuItem(gotext.Get("Clear"), "")
	pfx_backupmnt := pfx.AddSubMenuItem(gotext.Get("Mount backup"), "")
	pfx_backupunmnt := pfx.AddSubMenuItem(gotext.Get("Unmount backup"), "")

	settings := systray.AddMenuItem(gotext.Get("Settings"), "")
	config := settings.AddSubMenuItem(title, "")
	apps_cfg := settings.AddSubMenuItem(gotext.Get("Apps settings"), "")
	winemgr := settings.AddSubMenuItem(gotext.Get("Wine manager"), "")
	update := settings.AddSubMenuItem(gotext.Get("Runtime updater"), "")
	winetricks := settings.AddSubMenuItem(gotext.Get("Winetricks"), "")
	init := settings.AddSubMenuItem(gotext.Get("Forced init"), "")
	shell := settings.AddSubMenuItem(gotext.Get("Open SHELL"), "")
	shrt := settings.AddSubMenuItem(gotext.Get("Shortcut"), "")
	shrt_create := shrt.AddSubMenuItem(gotext.Get("Create"), "")
	shrt_remove := shrt.AddSubMenuItem(gotext.Get("Remove"), "")
	info := settings.AddSubMenuItem(gotext.Get("Info"), "")
	usage := info.AddSubMenuItem(gotext.Get("Usage"), "")
	version := info.AddSubMenuItem(gotext.Get("Version"), "")

	exit := systray.AddMenuItem(gotext.Get("Exit"), "")

	go func() {
		first_run := true
		var old_lw_apps string
		var new_lw_apps string
		for {
			if new_lw_apps != old_lw_apps || first_run {
				first_run = false
				apps.Show()
				apps_cfg.Show()
				lw_apps = get_lw_apps()
				for _, app_name := range lw_apps {
					_, exist := apps_items[app_name]
					if !exist {
						item := apps.AddSubMenuItem(app_name, "")
						cfg := apps_cfg.AddSubMenuItem(app_name, "")
						done := make(chan struct{})
						apps_items[app_name] = lw_app{item, cfg, done}
						go func() {
							for {
								select {
								case <-item.ClickedCh:
									lwexec(fmt.Sprintf("-runapp \"%s\"", app_name))
								case <-done:
									return
								}
							}
						}()
						go func() {
							for {
								select {
								case <-cfg.ClickedCh:
									lwexec(fmt.Sprintf("-appcfg \"%s\"", app_name))
								case <-done:
									return
								}
							}
						}()
					}
				}
				for app_item_name, app := range apps_items {
					exist := false
					for _, app_name := range lw_apps {
						if app_item_name == app_name {
							exist = true
							break
						}
					}
					if !exist {
						app.item.Hide()
						app.cfg.Hide()
						close(app.done)
						close(app.item.ClickedCh)
						close(app.cfg.ClickedCh)
						delete(apps_items, app_item_name)
					}
				}
			}
			if len(lw_apps) == 0 {
				apps.Hide()
				apps_cfg.Hide()
			}
			old_lw_apps = ls_lw_apps()
			time.Sleep(time.Second * 1)
			new_lw_apps = ls_lw_apps()
		}
	}()

	go func() {
		for {
			select {
			case <-open_luxwine.ClickedCh:
				lwexec("")
			case <-open_debug.ClickedCh:
				lwexec("-debug")

			case <-wine_explorer.ClickedCh:
				lwexec("-explorer")
			case <-wine_taskmgr.ClickedCh:
				lwexec("-taskmgr")
			case <-wine_cmd.ClickedCh:
				lwexec("-cmd")
			case <-wine_control.ClickedCh:
				lwexec("-control")
			case <-wine_cfg.ClickedCh:
				lwexec("-winecfg")
			case <-wine_regedit.ClickedCh:
				lwexec("-regedit")
			case <-wine_uninstaller.ClickedCh:
				lwexec("-uninstaller")

			case <-kill_exe.ClickedCh:
				lwexec("-killexe")
			case <-kill_wine.ClickedCh:
				lwexec("-killwine")
			case <-kill_shell.ClickedCh:
				lwexec("-killshell")
			case <-kill_all.ClickedCh:
				lwexec("-exit")

			case <-pfx_open.ClickedCh:
				lwexec("-openpfx")
			case <-pfx_backup.ClickedCh:
				lwexec("-pfxbackup")
			case <-pfx_restore.ClickedCh:
				lwexec("-pfxrestore")
			case <-pfx_clear.ClickedCh:
				lwexec("-clearpfx")
			case <-pfx_backupmnt.ClickedCh:
				lwexec("-backupmnt")
			case <-pfx_backupunmnt.ClickedCh:
				lwexec("-backupunmnt")

			case <-config.ClickedCh:
				lwexec("-config")
			case <-winemgr.ClickedCh:
				lwexec("-winemgr")
			case <-update.ClickedCh:
				lwexec("-update")
			case <-winetricks.ClickedCh:
				lwexec("-winetricks")
			case <-init.ClickedCh:
				lwexec("-init")
			case <-shell.ClickedCh:
				lwexec("-shell")
			case <-shrt_create.ClickedCh:
				lwexec("-shortcut")
			case <-shrt_remove.ClickedCh:
				lwexec("-rmapp")
			case <-usage.ClickedCh:
				lwexec("-help")
			case <-version.ClickedCh:
				lwexec("-version")

			case <-exit.ClickedCh:
				systray.Quit()
				return

			case <-stop:
				systray.Quit()
				return
			}
		}
	}()
}

func main() {
	load_translation(detect_lang())
	systray.Run(tray, on_exit)
}
