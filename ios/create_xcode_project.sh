#!/bin/bash

set -e

echo "🎨 Words with Toddlers - iOS Project Setup"
echo "==========================================="
echo ""

# Check if Xcode is installed
if ! command -v xcodebuild &> /dev/null; then
    echo "❌ Error: Xcode is not installed or not in PATH"
    echo "Please install Xcode from the App Store"
    exit 1
fi

echo "✅ Xcode found: $(xcodebuild -version | head -1)"
echo ""

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_DIR"

echo "📁 Creating Xcode project structure..."

# Create the project directory structure
mkdir -p "WordsWithToddlers.xcodeproj"

# Create the project.pbxproj file using xcodebuild
echo "🔨 Generating Xcode project file..."

cat > "generate_project.rb" << 'RUBY'
require 'xcodeproj'

project_path = 'WordsWithToddlers.xcodeproj'
project = Xcodeproj::Project.new(project_path)

# Create main target
target = project.new_target(:application, 'WordsWithToddlers', :ios, '17.0')

# Add files
main_group = project.main_group

# Models
models_group = main_group.new_group('Models', 'WordsWithToddlers/Models')
Dir.glob('WordsWithToddlers/Models/*.swift').each do |file|
  file_ref = models_group.new_reference(file)
  target.add_file_references([file_ref])
end

# Views
views_group = main_group.new_group('Views', 'WordsWithToddlers/Views')
Dir.glob('WordsWithToddlers/Views/*.swift').each do |file|
  file_ref = views_group.new_reference(file)
  target.add_file_references([file_ref])
end

# Services
services_group = main_group.new_group('Services', 'WordsWithToddlers/Services')
Dir.glob('WordsWithToddlers/Services/*.swift').each do |file|
  file_ref = services_group.new_reference(file)
  target.add_file_references([file_ref])
end

# Utilities
utilities_group = main_group.new_group('Utilities', 'WordsWithToddlers/Utilities')
Dir.glob('WordsWithToddlers/Utilities/*.swift').each do |file|
  file_ref = utilities_group.new_reference(file)
  target.add_file_references([file_ref])
end

# App file
app_file = main_group.new_reference('WordsWithToddlers/WordsWithToddlersApp.swift')
target.add_file_references([app_file])

# Resources
resources_group = main_group.new_group('Resources', 'WordsWithToddlers/Resources')
word_lists_ref = resources_group.new_reference('WordsWithToddlers/Resources/word_lists')
sounds_ref = resources_group.new_reference('WordsWithToddlers/Resources/sounds')
target.add_resources([word_lists_ref, sounds_ref])

# Assets
assets_ref = main_group.new_reference('WordsWithToddlers/Assets.xcassets')
target.add_resources([assets_ref])

# Build settings
target.build_configurations.each do |config|
  config.build_settings['PRODUCT_BUNDLE_IDENTIFIER'] = 'com.wecodefire.WordsWithToddlers'
  config.build_settings['MARKETING_VERSION'] = '1.0'
  config.build_settings['CURRENT_PROJECT_VERSION'] = '1'
  config.build_settings['INFOPLIST_FILE'] = 'WordsWithToddlers/Info.plist'
  config.build_settings['SWIFT_VERSION'] = '5.9'
  config.build_settings['TARGETED_DEVICE_FAMILY'] = '1,2'
  config.build_settings['IPHONEOS_DEPLOYMENT_TARGET'] = '17.0'
end

project.save
RUBY

# Try to generate using xcodeproj gem
if command -v gem &> /dev/null && gem list xcodeproj -i &> /dev/null; then
    echo "Using xcodeproj gem to generate project..."
    ruby generate_project.rb
    rm generate_project.rb
    echo "✅ Project generated successfully!"
else
    echo "⚠️  xcodeproj gem not found"
    rm generate_project.rb
    echo ""
    echo "Please create the project manually in Xcode:"
    echo "1. Open Xcode"
    echo "2. File → New → Project"
    echo "3. Choose iOS → App"
    echo "4. Product Name: WordsWithToddlers"
    echo "5. Interface: SwiftUI"
    echo "6. Language: Swift"
    echo "7. Save in: $PROJECT_DIR"
    echo "8. Add all Swift files from Models, Views, Services, Utilities"
    echo "9. Add Resources folder with word_lists and sounds"
    echo ""
    exit 1
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "To open the project in Xcode:"
echo "  open WordsWithToddlers.xcodeproj"
echo ""
echo "Or run:"
echo "  xcodebuild -project WordsWithToddlers.xcodeproj -scheme WordsWithToddlers -destination 'platform=iOS Simulator,name=iPhone 15' build"
echo ""
RUBY