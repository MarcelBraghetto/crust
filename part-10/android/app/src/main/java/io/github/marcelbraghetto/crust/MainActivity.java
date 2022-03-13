package io.github.marcelbraghetto.crust;

import org.libsdl.app.SDLActivity;

public class MainActivity extends SDLActivity {
    @Override
    protected String[] getLibraries() {
        return new String[]{
                "hidapi",
                "SDL2",
                "SDL2_image",
                "crustlib"
        };
    }
}
