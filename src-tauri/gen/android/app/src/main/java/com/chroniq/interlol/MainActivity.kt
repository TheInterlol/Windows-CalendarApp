package com.chroniq.interlol

import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import com.chroniq.interlol.TauriActivity // <-- TADY JE TA OPRAVA

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }
}