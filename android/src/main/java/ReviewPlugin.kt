package de.stecug.tauri.plugin.review

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import com.google.android.play.core.review.ReviewManagerFactory

@InvokeArg
class RequestReviewArgs

@TauriPlugin
class ReviewPlugin(private val activity: Activity) : Plugin(activity) {
    
    private val reviewManager by lazy {
        ReviewManagerFactory.create(activity)
    }

    @Command
    fun requestReview(invoke: Invoke) {
        val request = reviewManager.requestReviewFlow()
        
        request.addOnCompleteListener { task ->
            if (task.isSuccessful) {
                val reviewInfo = task.result
                val flow = reviewManager.launchReviewFlow(activity, reviewInfo)
                
                flow.addOnCompleteListener { flowTask ->
                    if (flowTask.isSuccessful) {
                        invoke.resolve(JSObject().put("success", true))
                    } else {
                        invoke.reject("Failed to launch review flow: ${flowTask.exception?.message}")
                    }
                }
            } else {
                invoke.reject("Failed to request review: ${task.exception?.message}")
            }
        }
        
        request.addOnFailureListener { exception ->
            invoke.reject("Review request failed: ${exception.message}")
        }
    }
}
