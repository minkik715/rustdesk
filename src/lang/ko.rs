lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "상태"),
        ("Your Desktop", "내 컴퓨터"),
        ("desk_tip", "아래 ID와 비밀번호를 통해 내 컴퓨터로 접속할 수 있습니다."),
        ("Password", "비밀번호"),
        ("Ready", "준비"),
        ("Established", "연결됨"),
        ("connecting_status", "RustDesk 네트워크로 연결중입니다..."),
        ("Enable Service", "서비스 활성화"),
        ("Start Service", "서비스 시작"),
        ("Service is running", "서비스 동작중"),
        ("Service is not running", "서비스가 동작하고 있지 않습니다"),
        ("not_ready_status", "준비되지 않음. 연결을 확인해주시길 바랍니다."),
        ("Control Remote Desktop", "원격 데스크탑 제어"),
        ("Transfer File", "파일 전송"),
        ("Connect", "접속하기"),
        ("Recent Sessions", "최근 세션"),
        ("Address Book", "세션 주소록"),
        ("Confirmation", "확인"),
        ("TCP Tunneling", "TCP 터널링"),
        ("Remove", "삭제"),
        ("Refresh random password", "랜덤 비밀번호 새로고침"),
        ("Set your own password", "개인 비밀번호 설정"),
        ("Enable Keyboard/Mouse", "키보드/마우스 활성화"),
        ("Enable Clipboard", "클립보드 활성화"),
        ("Enable File Transfer", "파일 전송 활성화"),
        ("Enable TCP Tunneling", "TCP 터널링 활성화"),
        ("IP Whitelisting", "IP 화이트리스트"),
        ("ID/Relay Server", "중계/Relay 서버"),
        ("Import Server Config", "서버 설정 가져오기"),
        ("Export Server Config", ""),
        ("Import server configuration successfully", "서버 설정 가져오기 성공"),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", "잘못된 서버 설정"),
        ("Clipboard is empty", "클립보드가 비어있습니다"),
        ("Stop service", "서비스 중단"),
        ("Change ID", "ID 변경"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "a-z, A-Z, 0-9, _(밑줄 문자)만 입력 가능합니다. 첫 문자는 a-z 혹은 A-Z로 시작해야 합니다. 길이는 6 ~ 16글자가 요구됩니다."),
        ("Website", "웹사이트"),
        ("About", "정보"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "음소거"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", "메인"),
        ("Audio Input", "오디오 입력"),
        ("Enhancements", ""),
        ("Hardware Codec", "하드웨어 코덱"),
        ("Adaptive bitrate", "가변 비트레이트"),
        ("ID Server", "중계 서버 주소"),
        ("Relay Server", "Relay 서버"),
        ("API Server", "API 서버"),
        ("Key", "암호 키"),
        ("invalid_http", "다음과 같이 시작해야 합니다. http:// 또는 https://"),
        ("Invalid IP", "유효하지 않은 IP"),
        ("Invalid format", "유효하지 않은 형식"),
        ("server_not_support", "해당 서버가 아직 지원하지 않습니다"),
        ("Not available", "불가능"),
        ("Too frequent", "너무 잦은 시도"),
        ("Cancel", "취소"),
        ("Skip", "넘기기"),
        ("Close", "닫기"),
        ("Retry", "재시도"),
        ("OK", "확인"),
        ("Password Required", "비밀번호 입력"),
        ("Please enter your password", "비밀번호를 입력해주세요"),
        ("Remember password", "비밀번호 저장"),
        ("Wrong Password", "틀린 비밀번호"),
        ("Do you want to enter again?", "다시 접속하시겠습니까?"),
        ("Connection Error", "연결 에러"),
        ("Error", "에러"),
        ("Reset by the peer", "다른 접속자에 의해 초기화됨"),
        ("Connecting...", "연결중..."),
        ("Connection in progress. Please wait.", "연결중입니다. 잠시만 기다려주세요."),
        ("Please try 1 minute later", "1분 뒤 다시 시도해주세요"),
        ("Login Error", "로그인 에러"),
        ("Successful", "성공"),
        ("Connected, waiting for image...", "연결됨. 이미지를 기다리는중..."),
        ("Name", "이름"),
        ("Type", "유형"),
        ("Modified", "수정됨"),
        ("Size", "크기"),
        ("Show Hidden Files", "숨김 파일 보기"),
        ("Receive", "받기"),
        ("Send", "보내기"),
        ("Refresh File", "파일 새로고침"),
        ("Local", "로컬"),
        ("Remote", "원격"),
        ("Remote Computer", "원격 컴퓨터"),
        ("Local Computer", "로컬 컴퓨터"),
        ("Confirm Delete", "삭제 재확인"),
        ("Delete", "삭제"),
        ("Properties", "속성"),
        ("Multi Select", "다중 선택"),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", "빈 디렉터리"),
        ("empty_recent_tip", ""),
        ("Oops, no recent sessions!\nTime to plan a new one.", ""),
        ("Not an empty directory", "디렉터리가 비어있지 않습니다"),
        ("Are you sure you want to delete this file?", "정말로 해당 파일을 삭제하시겠습니까?"),
        ("Are you sure you want to delete this empty directory?", "정말로 비어있는 해당 디렉터리를 삭제하시겠습니까?"),
        ("Are you sure you want to delete the file of this directory?", "정말로 해당 파일 혹은 디렉터리를 삭제하시겠습니까?"),
        ("Do this for all conflicts", "모든 충돌에 대해 해당 작업 수행"),
        ("This is irreversible!", "해당 결정은 돌이킬 수 없습니다!"),
        ("Deleting", "삭제중"),
        ("files", "파일"),
        ("Waiting", "대기중"),
        ("Finished", "완료됨"),
        ("Speed", "속도"),
        ("Custom Image Quality", "이미지 품질 조정"),
        ("Privacy mode", "개인정보 보호 모드"),
        ("Block user input", "사용자 입력 차단"),
        ("Unblock user input", "사용자 입력 차단 해제"),
        ("Adjust Window", "화면 조정"),
        ("Original", "원본"),
        ("Shrink", "축소"),
        ("Stretch", "확대"),
        ("Scrollbar", ""),
        ("ScrollAuto", ""),
        ("Good image quality", "최적 이미지 품질"),
        ("Balanced", "균형"),
        ("Optimize reaction time", "반응 시간 최적화"),
        ("Custom", "직접 설정"),
        ("Show remote cursor", "원격 커서 보이기"),
        ("Show quality monitor", "품질 모니터 띄우기"),
        ("Disable clipboard", "클립보드 비활성화"),
        ("Lock after session end", "세션 종료 후 화면 잠금"),
        ("Insert", "입력"),
        ("Insert Lock", "입력 잠금"),
        ("Refresh", "새로고침"),
        ("ID does not exist", "ID가 존재하지 않습니다"),
        ("Failed to connect to rendezvous server", "rendezvous 서버에 접속을 실패하였습니다"),
        ("Please try later", "다시 시도해주세요"),
        ("Remote desktop is offline", "원격 데스크탑이 연결되어 있지 않습니다"),
        ("Key mismatch", "키가 일치하지 않습니다."),
        ("Timeout", "시간 초과"),
        ("Failed to connect to relay server", "relay 서버에 접속을 실패하였습니다"),
        ("Failed to connect via rendezvous server", "rendezvous 서버를 통한 접속에 실패하였습니다"),
        ("Failed to connect via relay server", "relay 서버를 통한 접속에 실패하였습니다"),
        ("Failed to make direct connection to remote desktop", "원격 데스크탑으로의 직접 연결 생성에 실패하였습니다"),
        ("Set Password", "비밀번호 설정"),
        ("OS Password", "OS 비밀번호"),
        ("install_tip", "UAC로 인해, RustDesk가 원격지일 때 일부 기능이 동작하지 않을 수 있습니다. UAC 문제를 방지하려면, 아래 버튼을 클릭하여 RustDesk를 시스템에 설치해주세요."),
        ("Click to upgrade", "클릭하여 업그레이드"),
        ("Click to download", "클릭하여 다운로드"),
        ("Click to update", "클릭하여 업데이트"),
        ("Configure", "구성"),
        ("config_acc", "내 컴퓨터를 원격으로 제어하기 전에, RustDesk에게 \"Accessibility (접근성)\" 권한을 부여해야 합니다."),
        ("config_screen", "내 컴퓨터를 원격으로 제어하기 전에, RustDesk에게 \"Screen Recording (화면 녹화)\" 권한을 부여해야 합니다."),
        ("Installing ...", "설치중 ..."),
        ("Install", "설치하기"),
        ("Installation", "설치"),
        ("Installation Path", "설치 경로"),
        ("Create start menu shortcuts", "시작 메뉴에 바로가기 생성"),
        ("Create desktop icon", "데스크탑 아이콘 생성"),
        ("agreement_tip", "설치를 시작하기 전에, 라이선스 약관에 동의를 해야합니다."),
        ("Accept and Install", "동의 및 설치"),
        ("End-user license agreement", "최종 사용자 라이선스 약관 동의"),
        ("Generating ...", "생성중 ..."),
        ("Your installation is lower version.", "설치 버전이 최신 버전이 아닙니다."),
        ("not_close_tcp_tip", "연결을 사용하는 동안 이 창을 끄지 마세요"),
        ("Listening ...", "연결 대기중 ..."),
        ("Remote Host", "원격 호스트"),
        ("Remote Port", "원격 포트"),
        ("Action", "액션"),
        ("Add", "추가"),
        ("Local Port", "로컬 포트"),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", "빠른 접속을 위해, 당신의 서버를 설정하세요"),
        ("Too short, at least 6 characters.", "너무 짧습니다, 최소 6글자 이상 입력해주세요."),
        ("The confirmation is not identical.", "확인용 입력이 일치하지 않습니다."),
        ("Permissions", "권한"),
        ("Accept", "수락"),
        ("Dismiss", "거부"),
        ("Disconnect", "연결 종료"),
        ("Allow using keyboard and mouse", "키보드와 마우스 허용"),
        ("Allow using clipboard", "클립보드 허용"),
        ("Allow hearing sound", "소리 듣기 허용"),
        ("Allow file copy and paste", "파일 복사 및 붙여넣기 허용"),
        ("Connected", "연결됨"),
        ("Direct and encrypted connection", "암호화된 직접 연결"),
        ("Relayed and encrypted connection", "암호화된 릴레이 연결"),
        ("Direct and unencrypted connection", "암호화되지 않은 직접 연결"),
        ("Relayed and unencrypted connection", "암호화되지 않은 릴레이 연결"),
        ("Enter Remote ID", "원격지 ID를 입력하세요"),
        ("Enter your password", "비밀번호를 입력하세요"),
        ("Logging in...", "로그인 중..."),
        ("Enable RDP session sharing", "RDP 세션 공유를 활성화하세요"),
        ("Auto Login", "자동 로그인"),
        ("Enable Direct IP Access", "IP 직접 접근 활성화하세요"),
        ("Rename", "이름 변경"),
        ("Space", "공간"),
        ("Create Desktop Shortcut", "데스크탑 바로가기 생성"),
        ("Change Path", "경로 변경"),
        ("Create Folder", "폴더 생성"),
        ("Please enter the folder name", "폴더명을 입력해주세요"),
        ("Fix it", "문제 해결"),
        ("Warning", "경고"),
        ("Login screen using Wayland is not supported", "Wayland를 사용한 로그인 화면이 지원되지 않습니다"),
        ("Reboot required", "재부팅이 필요합니다"),
        ("Unsupported display server", "지원하지 않는 디스플레이 서버"),
        ("x11 expected", "x11 예상됨"),
        ("Port", ""),
        ("Settings", "설정"),
        ("Username", "사용자명"),
        ("Invalid port", "유효하지 않은 포트"),
        ("Closed manually by the peer", "다른 사용자에 의해 종료됨"),
        ("Enable remote configuration modification", "원격 구성 변경 활성화"),
        ("Run without install", "설치 없이 실행"),
        ("Connect via relay", ""),
        ("Always connect via relay", "항상 relay를 통해 접속하기"),
        ("whitelist_tip", "화이트리스트에 있는 IP만 현 데스크탑에 접속 가능합니다"),
        ("Login", "로그인"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "로그아웃"),
        ("Tags", "태그"),
        ("Search ID", "ID 검색"),
        ("whitelist_sep", "다음 글자로 구분합니다. ',(콤마) ;(세미콜론) 띄어쓰기 혹은 줄바꿈'"),
        ("Add ID", "ID 추가"),
        ("Add Tag", "태그 추가"),
        ("Unselect all tags", "모든 태그 선택 해제"),
        ("Network error", "네트워크 에러"),
        ("Username missed", "사용자명 누락"),
        ("Password missed", "비밀번호 누락"),
        ("Wrong credentials", "틀린 인증 정보"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "태그 수정"),
        ("Forget Password", "패스워드 기억하지 않기"),
        ("Favorites", "즐겨찾기"),
        ("Add to Favorites", "즐겨찾기에 추가"),
        ("Remove from Favorites", "즐겨찾기에서 삭제"),
        ("Empty", "비어 있음"),
        ("Invalid folder name", "유효하지 않은 폴더명"),
        ("Socks5 Proxy", "Socks5 프록시"),
        ("Hostname", "호스트명"),
        ("Discovered", "찾음"),
        ("install_daemon_tip", "부팅된 이후 시스템 서비스에 설치해야 합니다."),
        ("Remote ID", "원격지 ID"),
        ("Paste", "붙여넣기"),
        ("Paste here?", "여기에 붙여넣겠습니까?"),
        ("Are you sure to close the connection?", "정말로 연결을 종료하시겠습니까?"),
        ("Download new version", "최신 버전 다운로드"),
        ("Touch mode", "터치 모드"),
        ("Mouse mode", "마우스 모드"),
        ("One-Finger Tap", "한 손가락 탭"),
        ("Left Mouse", "왼쪽 마우스"),
        ("One-Long Tap", "길게 누르기"),
        ("Two-Finger Tap", "두 손가락 탭"),
        ("Right Mouse", "오른쪽 마우스"),
        ("One-Finger Move", "한 손가락 이동"),
        ("Double Tap & Move", "두 번 탭 하고 이동"),
        ("Mouse Drag", "마우스 드래그"),
        ("Three-Finger vertically", "세 손가락 세로로"),
        ("Mouse Wheel", "마우스 휠"),
        ("Two-Finger Move", "두 손가락 이동"),
        ("Canvas Move", "캔버스 이동"),
        ("Pinch to Zoom", "확대/축소"),
        ("Canvas Zoom", "캔버스 확대"),
        ("Reset canvas", "캔버스 초기화"),
        ("No permission of file transfer", "파일 전송 권한이 없습니다"),
        ("Note", "노트"),
        ("Connection", "연결"),
        ("Share Screen", "화면 공유"),
        ("Chat", "채팅"),
        ("Total", "총합"),
        ("items", "개체"),
        ("Selected", "선택됨"),
        ("Screen Capture", "화면 캡처"),
        ("Input Control", "입력 제어"),
        ("Audio Capture", "오디오 캡처"),
        ("File Connection", "파일 전송"),
        ("Screen Connection", "화면 전송"),
        ("Do you accept?", "동의하십니까?"),
        ("Open System Setting", "시스템 설정 열기"),
        ("How to get Android input permission?", "안드로이드 입력 권한에 어떻게 접근합니까?"),
        ("android_input_permission_tip1", "원격지로서 마우스나 터치를 통해 Android 장치를 제어하려면 RustDesk에서 \"Accessibility (접근성)\" 서비스 사용을 허용해야 합니다."),
        ("android_input_permission_tip2", "시스템 설정 페이지로 이동하여 [설치된 서비스]에서 [RustDesk Input] 서비스를 켜십시오."),
        ("android_new_connection_tip", "현재 장치의 새로운 제어 요청이 수신되었습니다."),
        ("android_service_will_start_tip", "\"화면 캡처\"를 켜면 서비스가 자동으로 시작되어 다른 장치에서 사용자 장치에 대한 연결을 요청할 수 있습니다."),
        ("android_stop_service_tip", "서비스를 종료하면 모든 연결이 자동으로 닫힙니다."),
        ("android_version_audio_tip", "현재 Android 버전은 오디오 캡처를 지원하지 않습니다. Android 10 이상으로 업그레이드하십시오."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "계정"),
        ("Overwrite", "덮어쓰기"),
        ("This file exists, skip or overwrite this file?", "해당 파일이 이미 존재합니다, 넘어가거나 덮어쓰시겠습니까?"),
        ("Quit", "종료"),
        ("Help", "지원"),
        ("Failed", "실패"),
        ("Succeeded", "성공"),
        ("Someone turns on privacy mode, exit", "누군가가 개인정보 보호 모드를 활성화하여 종료됩니다"),
        ("Unsupported", "지원되지 않음"),
        ("Peer denied", "다른 사용자에 의해 거부됨"),
        ("Please install plugins", "플러그인을 설치해주세요"),
        ("Peer exit", "다른 사용자가 나감"),
        ("Failed to turn off", "종료에 실패함"),
        ("Turned off", "종료됨"),
        ("In privacy mode", "개인정보 보호 모드 진입"),
        ("Out privacy mode", "개인정보 보호 모드 나감"),
        ("Language", "언어"),
        ("Keep RustDesk background service", "RustDesk 백그라운드 서비스로 유지하기"),
        ("Ignore Battery Optimizations", "배터리 최적화 무시하기"),
        ("android_open_battery_optimizations_tip", "해당 기능을 비활성화하려면 RustDesk 응용 프로그램 설정 페이지로 이동하여 [배터리]에서 [제한 없음] 선택을 해제하십시오."),
        ("Start on Boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "연결이 허용되지 않음"),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", "영구 비밀번호 사용"),
        ("Use both passwords", "두 비밀번호 (임시/영구) 사용"),
        ("Set permanent password", "영구 비밀번호 설정"),
        ("Enable Remote Restart", "원격지 재시작 활성화"),
        ("Allow remote restart", "원격지 재시작 허용"),
        ("Restart Remote Device", "원격 기기 재시작"),
        ("Are you sure you want to restart", "정말로 재시작 하시겠습니까"),
        ("Restarting Remote Device", "원격 기기를 다시 시작하는중"),
        ("remote_restarting_tip", "원격 장치를 다시 시작하는 중입니다. 이 메시지 상자를 닫고 잠시 후 영구 비밀번호로 다시 연결하십시오."),
        ("Copied", ""),
        ("Exit Fullscreen", "전체 화면 종료"),
        ("Fullscreen", "전체화면"),
        ("Mobile Actions", "모바일 액션"),
        ("Select Monitor", "모니터 선택"),
        ("Control Actions", "제어 작업"),
        ("Display Settings", "화면 설정"),
        ("Ratio", "비율"),
        ("Image Quality", "이미지 품질"),
        ("Scroll Style", "스크롤 스타일"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "직접 연결"),
        ("Relay Connection", "릴레이 연결"),
        ("Secure Connection", "보안 연결"),
        ("Insecure Connection", "안전하지 않은 연결"),
        ("Scale original", "원래 크기"),
        ("Scale adaptive", "맞는 창"),
        ("General", "일반"),
        ("Security", "보안"),
        ("Theme", "테마"),
        ("Dark Theme", ""),
        ("Light Theme", ""),
        ("Dark", "다크 테마"),
        ("Light", "밝은 테마"),
        ("Follow System", "시스템에 따라 설정"),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", "보안 설정 보호 해제"),
        ("Enable Audio", "오디오 활성화"),
        ("Unlock Network Settings", "네트워크 설정 보호 해제"),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Apply", "적용"),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", "오디오 입력 장치"),
        ("Use IP Whitelisting", ""),
        ("Network", "네트워크"),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", "화면 녹화"),
        ("Directory", ""),
        ("Automatically record incoming sessions", "연결시 자동 녹화"),
        ("Change", "변경"),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable Recording Session", "녹화 활성화"),
        ("Allow recording session", ""),
        ("Enable LAN Discovery", ""),
        ("Deny LAN Discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", "여러개 탭 닫기 전에 확인"),
        ("Keyboard Settings", ""),
        ("Full Access", "전체 권한"),
        ("Screen Share", "화면만 공유"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland는 Ubuntu 21.04 이상 버전이 필요합니다."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland에는 더 높은 버전의 Linux 배포판이 필요합니다. X11 데스크탑을 시도하거나 OS를 변경하십시오."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "공유할 화면을 선택하십시오(피어 측에서 작동)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", "화면에 맞는 커서 크기"),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", "일회용 비밀번호"),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", "원격 제어 스위칭"),
        ("Please confirm if you want to share your desktop?", "정말로 당신의 컴퓨터를 원격으로 공유하겠습니까?"),
        ("Display", "화면"),
        ("Default View Style", "기본 화면 설정"),
        ("Default Image Quality", "기본 이미지 화질"),
        ("Default Codec", "기본 코덱"),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", "그 외 기본 설정"),
        ("Voice call", "보이스 콜"),
        ("Text chat", "문자 채팅"),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", "코덱 설정"),
        ("Resolution", "해상도 설정"),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("install_cert_tip", ""),
        ("confirm_install_cert_tip", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", "뷰 모드(매니저 입력 차단)"),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", "시스템 사운드"),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", "서비스"),
        ("Start", "시작"),
        ("Stop", "중지"),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", "연결시 새 탭에서 열기"),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", "마우스 휠 상하 반대로 제어"),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("switch_display_elevated_connections_tip", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color(4:4:4)", ""),
    ].iter().cloned().collect();
}
