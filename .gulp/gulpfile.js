var gulp = require('gulp');
var uglify = require('gulp-uglify');
var source = require('vinyl-source-stream');
var browserify = require('browserify');
var reactify = require('reactify');

var path = {
  MINIFIED_OUT: 'deps.min.js',
  OUT: 'deps.js',
  DEST: '../',
  DEST_SRC: '../',
  DEST_BUILD: '../lib',
  ENTRY_POINT: './deps.js'
};

gulp.task('build', function(){
  browserify({
    entries: [path.ENTRY_POINT],
    transform: [reactify]
  })
    .bundle()
    //.pipe(source(path.MINIFIED_OUT))
    .pipe(source(path.OUT))
    //.pipe(streamify(uglify(path.MINIFIED_OUT)))
    .pipe(gulp.dest(path.DEST_BUILD));
});

gulp.task('default', ['build']);