// Dashboard JavaScript - Chart.js integration
// Generates real-looking charts with cat-themed useless data

document.addEventListener('DOMContentLoaded', function() {
    console.log('🐱 Verdant™ Analytics Dashboard Initialized');
    
    // Chart.js default configuration
    Chart.defaults.font.family = '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif';
    Chart.defaults.color = '#64748b';
    
    const primaryColor = '#10b981';
    const primaryDark = '#059669';
    const secondaryColor = '#6366f1';
    const grayColor = '#94a3b8';
    
    // Purr-formance Trend Chart
    const purrformanceCtx = document.getElementById('purrformanceChart');
    if (purrformanceCtx) {
        new Chart(purrformanceCtx, {
            type: 'line',
            data: {
                labels: ['Week 1', 'Week 2', 'Week 3', 'Week 4', 'Week 5', 'Week 6', 'Week 7', 'Week 8'],
                datasets: [{
                    label: 'Purr-formance Score',
                    data: [78, 82, 79, 85, 88, 84, 89, 87],
                    borderColor: primaryColor,
                    backgroundColor: primaryColor + '20',
                    tension: 0.4,
                    fill: true,
                    pointRadius: 4,
                    pointBackgroundColor: primaryColor,
                    pointHoverRadius: 6
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: true,
                plugins: {
                    legend: {
                        display: false
                    },
                    tooltip: {
                        backgroundColor: 'rgba(15, 23, 42, 0.9)',
                        padding: 12,
                        titleFont: {
                            size: 14,
                            weight: '600'
                        },
                        bodyFont: {
                            size: 13
                        },
                        callbacks: {
                            label: function(context) {
                                return 'Score: ' + context.parsed.y.toFixed(1);
                            }
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: false,
                        min: 70,
                        max: 100,
                        grid: {
                            color: '#f1f5f9'
                        }
                    },
                    x: {
                        grid: {
                            display: false
                        }
                    }
                }
            }
        });
    }
    
    // Box Occupancy by Hour Chart
    const boxOccupancyCtx = document.getElementById('boxOccupancyChart');
    if (boxOccupancyCtx) {
        new Chart(boxOccupancyCtx, {
            type: 'bar',
            data: {
                labels: ['12AM', '3AM', '6AM', '9AM', '12PM', '3PM', '6PM', '9PM'],
                datasets: [{
                    label: 'Box Occupancy Rate',
                    data: [15, 8, 25, 65, 78, 72, 55, 40],
                    backgroundColor: primaryColor,
                    borderRadius: 6,
                    borderSkipped: false
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: true,
                plugins: {
                    legend: {
                        display: false
                    },
                    tooltip: {
                        backgroundColor: 'rgba(15, 23, 42, 0.9)',
                        padding: 12,
                        callbacks: {
                            label: function(context) {
                                return 'Occupancy: ' + context.parsed.y + '%';
                            }
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        max: 100,
                        grid: {
                            color: '#f1f5f9'
                        },
                        ticks: {
                            callback: function(value) {
                                return value + '%';
                            }
                        }
                    },
                    x: {
                        grid: {
                            display: false
                        }
                    }
                }
            }
        });
    }
    
    // Nap Correlation Chart
    const napCorrelationCtx = document.getElementById('napCorrelationChart');
    if (napCorrelationCtx) {
        new Chart(napCorrelationCtx, {
            type: 'scatter',
            data: {
                datasets: [{
                    label: 'Code Quality',
                    data: [
                        { x: 4, y: 65 },
                        { x: 5, y: 72 },
                        { x: 6, y: 85 },
                        { x: 7, y: 92 },
                        { x: 8, y: 88 },
                        { x: 9, y: 78 },
                        { x: 10, y: 68 }
                    ],
                    backgroundColor: primaryColor,
                    borderColor: primaryDark,
                    pointRadius: 8,
                    pointHoverRadius: 10
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: true,
                plugins: {
                    legend: {
                        display: false
                    },
                    tooltip: {
                        backgroundColor: 'rgba(15, 23, 42, 0.9)',
                        padding: 12,
                        callbacks: {
                            label: function(context) {
                                return 'Nap: ' + context.parsed.x + 'h, Quality: ' + context.parsed.y;
                            }
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: false,
                        min: 50,
                        max: 100,
                        title: {
                            display: true,
                            text: 'Code Quality Score',
                            font: {
                                weight: '600'
                            }
                        },
                        grid: {
                            color: '#f1f5f9'
                        }
                    },
                    x: {
                        title: {
                            display: true,
                            text: 'Nap Duration (hours)',
                            font: {
                                weight: '600'
                            }
                        },
                        grid: {
                            color: '#f1f5f9'
                        }
                    }
                }
            }
        });
    }
    
    // Territorial Coverage Chart
    const territorialCtx = document.getElementById('territorialChart');
    if (territorialCtx) {
        new Chart(territorialCtx, {
            type: 'line',
            data: {
                labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
                datasets: [
                    {
                        label: 'Morning (6AM-12PM)',
                        data: [45, 52, 48, 55, 50, 30, 25],
                        borderColor: primaryColor,
                        backgroundColor: primaryColor + '40',
                        tension: 0.4,
                        fill: true
                    },
                    {
                        label: 'Afternoon (12PM-6PM)',
                        data: [75, 78, 80, 82, 77, 45, 40],
                        borderColor: secondaryColor,
                        backgroundColor: secondaryColor + '40',
                        tension: 0.4,
                        fill: true
                    },
                    {
                        label: 'Evening (6PM-12AM)',
                        data: [55, 58, 60, 62, 65, 70, 55],
                        borderColor: grayColor,
                        backgroundColor: grayColor + '40',
                        tension: 0.4,
                        fill: true
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: true,
                plugins: {
                    legend: {
                        position: 'bottom',
                        labels: {
                            usePointStyle: true,
                            padding: 15,
                            font: {
                                size: 12,
                                weight: '600'
                            }
                        }
                    },
                    tooltip: {
                        backgroundColor: 'rgba(15, 23, 42, 0.9)',
                        padding: 12,
                        callbacks: {
                            label: function(context) {
                                return context.dataset.label + ': ' + context.parsed.y + '%';
                            }
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        max: 100,
                        grid: {
                            color: '#f1f5f9'
                        },
                        ticks: {
                            callback: function(value) {
                                return value + '%';
                            }
                        }
                    },
                    x: {
                        grid: {
                            display: false
                        }
                    }
                },
                interaction: {
                    mode: 'index',
                    intersect: false
                }
            }
        });
    }
    
    // Animate metric values on load
    animateValue('purrformance-score', 0, 87.3, 1500);
    animateValue('box-occupancy', 0, 73, 1500, '%');
    animateValue('zoomie-frequency', 0, 42, 1500);
    animateValue('treat-rate', 0, 3.2, 1500);
    
    function animateValue(id, start, end, duration, suffix = '') {
        const element = document.getElementById(id);
        if (!element) return;
        
        const range = end - start;
        const increment = range / (duration / 16);
        let current = start;
        
        const timer = setInterval(() => {
            current += increment;
            if ((increment > 0 && current >= end) || (increment < 0 && current <= end)) {
                current = end;
                clearInterval(timer);
            }
            
            if (suffix === '%') {
                element.textContent = Math.round(current) + suffix;
            } else if (end % 1 !== 0) {
                element.textContent = current.toFixed(1);
            } else {
                element.textContent = Math.round(current);
            }
        }, 16);
    }
});
