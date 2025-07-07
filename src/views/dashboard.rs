use chrono::Utc;
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    let (isConnected, _setIsConnected) = signal(false);
    let (lastUpdated, _setLastUpdated) = signal(Utc::now());
    let (healthyCount, _setHealthyCount) = signal(4);
    let (degradedCount, _setDegradedCount) = signal(0);
    let (criticalCount, _setCriticalCount) = signal(0);
    let (healthScore, _setHealthScore) = signal(100);

    view! {
        <div class="min-h-screen bg-white-50 p-6 rounded-xl shadow-md">
            <div class="bg-gray-700 shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center py-6">
                        <h1 class="text-3xl font-bold text-white-900">System Health Dashboard</h1>
                        <div class="flex items-center space-x-4">
                            <div class="flex items-center space-x-2">
                                <div class=if isConnected.get() {
                                    "w-3 h-3 rounded-full if bg-green-400"
                                } else {
                                    "w-3 h-3 rounded-full if bg-red-400"
                                }></div>
                                <span class="text-sm text-white-600">
                                    {if isConnected.get() {
                                        " Connected "
                                    } else {
                                        " Disconnected "
                                    }}
                                </span>
                            </div>
                            <span class="text-sm text-white-500">
                                Last updated: {lastUpdated.get().format("%H:%M:%S").to_string()}
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-green-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-green-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Healthy Services
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {healthyCount.get()}
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>

                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-yellow-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-yellow-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Degraded Services
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {degradedCount.get()}
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>

                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-red-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-red-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Critical Services
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {criticalCount.get()}
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>

                    <div class="bg-gray-700 rounded-lg shadow p-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <div class="w-8 h-8 bg-blue-100 rounded-md flex items-center justify-center">
                                    <svg
                                        class="w-5 h-5 text-blue-600"
                                        fill="currentColor"
                                        viewBox="0 0 20 20"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z"
                                            clip-rule="evenodd"
                                        ></path>
                                    </svg>
                                </div>
                            </div>
                            <div class="ml-5 w-0 flex-1">
                                <dl>
                                    <dt class="text-sm font-medium text-white-500 truncate">
                                        Overall Health Score
                                    </dt>
                                    <dd class="text-lg font-medium text-white-900">
                                        {healthScore.get()}%
                                    </dd>
                                </dl>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="md:col-span-2 bg-gray-700 rounded-xl p-4">
                    <h3 class="font-semibold mb-4 text-white">Services Status</h3>
                    <ul class="divide-y divide-gray-600">
                        <li class="flex justify-between py-2 items-center">
                            <div class="flex items-center gap-2">
                                <div class="w-3 h-3 bg-green-400 rounded-full"></div>
                                <div>
                                    <div class="font-medium">CF Backend</div>
                                    <div class="text-sm text-gray-400">Http</div>
                                </div>
                            </div>
                            <div class="text-right text-gray-300 text-sm">
                                <div>229 ms</div>
                                <div>20:29</div>
                            </div>
                        </li>
                        <li class="flex justify-between py-2 items-center">
                            <div class="flex items-center gap-2">
                                <div class="w-3 h-3 bg-green-400 rounded-full"></div>
                                <div>
                                    <div class="font-medium">CF Scheduler</div>
                                    <div class="text-sm text-gray-400">Http</div>
                                </div>
                            </div>
                            <div class="text-right text-gray-300 text-sm">
                                <div>556 ms</div>
                                <div>20:29</div>
                            </div>
                        </li>
                        <li class="flex justify-between py-2 items-center">
                            <div class="flex items-center gap-2">
                                <div class="w-3 h-3 bg-green-400 rounded-full"></div>
                                <div>
                                    <div class="font-medium">Audit Logger</div>
                                    <div class="text-sm text-gray-400">Http</div>
                                </div>
                            </div>
                            <div class="text-right text-gray-300 text-sm">
                                <div>839 ms</div>
                                <div>20:29</div>
                            </div>
                        </li>
                        <li class="flex justify-between py-2 items-center">
                            <div class="flex items-center gap-2">
                                <div class="w-3 h-3 bg-green-400 rounded-full"></div>
                                <div>
                                    <div class="font-medium">MAF Notifier</div>
                                    <div class="text-sm text-gray-400">Http</div>
                                </div>
                            </div>
                            <div class="text-right text-gray-300 text-sm">
                                <div>546 ms</div>
                                <div>20:29</div>
                            </div>
                        </li>
                    </ul>
                    <a href="#" class="text-blue-400 text-sm mt-3 inline-block">
                        "View all services →"
                    </a>
                </div>

                <div class="flex flex-col gap-6">
                    <div class="bg-gray-700 p-4 rounded-xl">
                        <h3 class="font-semibold mb-4 text-white">Quick Actions</h3>
                        <div class="flex flex-col gap-2">
                            <button class="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded">
                                View Analytics
                            </button>
                            <button class="bg-gray-800 hover:bg-gray-700 text-white px-4 py-2 rounded border border-gray-600">
                                Manage Alerts
                            </button>
                            <button class="bg-gray-800 hover:bg-gray-700 text-white px-4 py-2 rounded border border-gray-600">
                                View Dependencies
                            </button>
                        </div>
                    </div>

                    <div class="bg-gray-700 p-4 rounded-xl">
                        <h3 class="font-semibold mb-2 text-white">Recent Alerts</h3>
                        <p class="text-sm text-gray-400">No recent alerts</p>
                    </div>

                    <div class="bg-gray-700 p-4 rounded-xl">
                        <h3 class="font-semibold mb-2 text-white">Health Trend</h3>
                        <div class="h-20 flex items-center justify-center text-gray-500 text-sm"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}