require 'libnotify'
Libnotify.show(:body => "hello", :summary => "world", :timeout => 2.5)

n = Libnotify.new do |notify|
  notify.summary    = "hello"
  notify.body       = "world"
  notify.timeout    = 1.5         # 1.5 (s), 1000 (ms), "2", nil, false
  notify.urgency    = :critical   # :low, :normal, :critical
  notify.append     = false       # default true - append onto existing notification
  notify.transient  = true        # default false - keep the notifications around after display
  notify.icon_path  = "/usr/share/icons/gnome/scalable/emblems/emblem-default.svg"
end


n.show!
