@set WL=
@set WL=%WL% --whitelist-function al.* 
@set WL=%WL% --whitelist-function alc.*
@set WL=%WL% --whitelist-type AL.*
@set WL=%WL% --whitelist-type ALC.*
@set WL=%WL% --whitelist-type LP.*
@set WL=%WL% --whitelist-type PFN.*
@set WL=%WL% --whitelist-var ALC.*
@set WL=%WL% --whitelist-var AL.*

@bindgen --rustified-enum * %WL% %OPENAL_SOFT_PATH%\include\AL\%1.h -o ../../src/%1_bindings.rs  -- -I %OPENAL_SOFT_PATH% -I %OPENAL_SOFT_PATH%\build -I %OPENAL_SOFT_PATH%\build\Debug -I %OPENAL_SOFT_PATH%\common -I %OPENAL_SOFT_PATH%\alc -I %OPENAL_SOFT_PATH%\include -x c++ -fms-compatibility -D AL_ALEXT_PROTOTYPES=1
